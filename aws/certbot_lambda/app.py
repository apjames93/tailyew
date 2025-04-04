import boto3
import os
import shutil
import logging
import certbot.main

logger = logging.getLogger()
logger.setLevel(logging.INFO)

def store_certificate_arn(cert_arn):
    logger.info(f"Storing ACM ARN to SSM")
    ssm = boto3.client('ssm', region_name=os.environ.get('ACM_REGION', 'us-east-1'))

    try:
        ssm.put_parameter(
            Name='/infra/acm_certificate_arn',
            Value=cert_arn,
            Type='String',
            Overwrite=True
        )
        logger.info("Successfully updated SSM Parameter Store")
    except Exception as e:
        logger.error(f"Failed to update SSM Parameter Store: {e}")
        raise

def get_existing_certificate(acm_client, domain_name):
    logger.info(f"Checking for existing ACM certificate for domain: {domain_name}")
    try:
        paginator = acm_client.get_paginator('list_certificates')
        for page_number, page in enumerate(paginator.paginate(Includes={'keyTypes': ['EC_prime256v1']}), start=1):
            logger.debug(f"Page {page_number}: {page['CertificateSummaryList']}")
            for cert_summary in page['CertificateSummaryList']:
                cert_arn = cert_summary['CertificateArn']
                logger.debug(f"Found Certificate Summary: {cert_summary}")

                cert_details = acm_client.describe_certificate(CertificateArn=cert_arn)
                logger.debug(f"Certificate Details: {cert_details}")

                cert_domain = cert_details['Certificate']['DomainName']
                alt_names = cert_details['Certificate'].get('SubjectAlternativeNames', [])
                if domain_name == cert_domain or domain_name in alt_names:
                    logger.info(f"Found matching certificate for domain {domain_name}: {cert_arn}")
                    return cert_arn

        logger.info(f"No matching certificate found for domain: {domain_name}")
        return None

    except Exception as e:
        logger.error(f"Error while retrieving certificates: {e}")
        raise

def handler(event, context):
    logger.info("Starting Certbot Lambda Function...")

    # Validate environment variables
    required_env_vars = ['DOMAIN_NAME', 'EMAIL', 'CERT_BUCKET', 'ACM_REGION']
    for var in required_env_vars:
        if var not in os.environ:
            logger.error(f"Missing required environment variable: {var}")
            raise ValueError(f"Missing required environment variable: {var}")

    domain_name = os.environ['DOMAIN_NAME']
    email = os.environ['EMAIL']
    cert_bucket = os.environ['CERT_BUCKET']
    acm_region = os.environ['ACM_REGION']

    # Set up temporary directory
    tmp_dir = "/tmp/certbot"
    os.makedirs(tmp_dir, exist_ok=True)

    try:
        # Toggle staging dynamically
        staging_flag = "--staging" if os.environ.get("IS_PRODUCTION", "false").lower() != "true" else ""
        certbot_args = [
            "certonly", "--dns-route53",
            "-m", email, "--agree-tos", "--non-interactive",
            "--config-dir", tmp_dir, "--work-dir", tmp_dir, "--logs-dir", tmp_dir,
            f"-d {domain_name}", f"-d *.{domain_name}"
        ]
        if staging_flag:
            certbot_args.append(staging_flag)

        logger.info(f"Running Certbot with arguments: {' '.join(certbot_args)}")
        for attempt in range(3):  # Retry up to 3 times
            try:
                certbot.main.main(certbot_args)
                break
            except Exception as e:
                if attempt < 2:
                    logger.warning(f"Certbot failed. Retrying in 30 seconds... (Attempt {attempt + 1})")
                    time.sleep(30)
                else:
                    logger.error(f"Certbot failed after 3 attempts: {e}")
                    raise

        # Paths to certificates
        cert_path = f"{tmp_dir}/live/{domain_name}/fullchain.pem"
        key_path = f"{tmp_dir}/live/{domain_name}/privkey.pem"

        # Validate certificates
        with open(cert_path, 'r') as cert_file:
            cert_data = cert_file.read()
        cert_parts = cert_data.split("-----END CERTIFICATE-----")
        if len(cert_parts) < 2:
            raise ValueError("Invalid certificate chain: fullchain.pem does not contain multiple certificates.")

        leaf_cert = cert_parts[0] + "-----END CERTIFICATE-----\n"
        chain_cert = "".join([part + "-----END CERTIFICATE-----\n" for part in cert_parts[1:] if part.strip()])

        # Upload to S3
        s3 = boto3.client('s3')
        try:
            s3.upload_file(cert_path, cert_bucket, f"{domain_name}/fullchain.pem", ExtraArgs={'ServerSideEncryption': 'AES256'})
            s3.upload_file(key_path, cert_bucket, f"{domain_name}/privkey.pem", ExtraArgs={'ServerSideEncryption': 'AES256'})
            logger.info(f"Certificates uploaded to S3 bucket: {cert_bucket}")
        except Exception as e:
            logger.error(f"Failed to upload certificates to S3: {e}")
            raise

        # Check and update ACM certificate
        acm = boto3.client('acm', region_name=acm_region)
        existing_cert_arn = get_existing_certificate(acm, domain_name)

        with open(key_path, 'r') as key_file:
            private_key = key_file.read()

        if existing_cert_arn:
            logger.info(f"Updating existing certificate: {existing_cert_arn}")
            acm.import_certificate(
                Certificate=leaf_cert,
                CertificateChain=chain_cert,
                PrivateKey=private_key,
                CertificateArn=existing_cert_arn
            )
            cert_arn = existing_cert_arn
        else:
            logger.info(f"Creating new ACM certificate for domain: {domain_name} and wildcard subdomains")
            response = acm.import_certificate(
                Certificate=leaf_cert,
                CertificateChain=chain_cert,
                PrivateKey=private_key
            )
            cert_arn = response['CertificateArn']

        logger.info(f"Certificate imported to ACM: {cert_arn}")
        store_certificate_arn(cert_arn)

        # Remove files from S3
        try:
            s3.delete_object(Bucket=cert_bucket, Key=f"{domain_name}/fullchain.pem")
            s3.delete_object(Bucket=cert_bucket, Key=f"{domain_name}/privkey.pem")
            logger.info(f"Certificate files removed from S3 bucket: {cert_bucket}")
        except Exception as e:
            logger.error(f"Failed to delete certificates from S3: {e}")
            raise

        return {"message": f"Certificate successfully imported to ACM and applied to domain: {domain_name} and *.{domain_name}"}

    except Exception as e:
        logger.exception(f"An error occurred: {e}")
        raise

    finally:
        shutil.rmtree(tmp_dir, ignore_errors=True)
