
# Certbot Lambda Function

This project implements a **Lambda function** designed to automate the management of SSL/TLS certificates using **Certbot** and Amazon Certificate Manager (**ACM**). The function uses Certbot's **Route 53 plugin** to create or renew certificates for a given domain and uploads the certificates securely to **Amazon S3**.

---

## **Features**

- Automatically obtains and renews SSL/TLS certificates for a given domain.
- Uploads certificates to an S3 bucket with encryption.
- Imports certificates into Amazon Certificate Manager (ACM) for use with AWS services.
- Removes temporary files and uploaded certificates after processing.

---

## **Project Structure**

- **`app.py`**: Contains the Lambda handler logic for certificate issuance, storage, and ACM updates.
- **`requirements.txt`**: Lists the dependencies required for the Lambda function.

---

## **Prerequisites**

### **Environment Requirements**

Ensure the following tools are installed:
- **Python 3.9**
- **AWS SAM CLI**
- **Docker** (for local testing)


### **2. Build the Lambda Function**
Build the Lambda function using the SAM CLI:
```bash
make aws-sam-build
```

This will:
- Use the SAM CLI to build the function.
- Invoke the function locally using `sam local invoke CertbotFunction`.

---

## **Environment Variables**

The function requires the following environment variables:

| Variable      | Description                                      | Example Value                |
|---------------|--------------------------------------------------|------------------------------|
| `CERT_BUCKET` | Name of the S3 bucket for storing certificates.  | `data.tailyew.com`     |
| `DOMAIN_NAME` | The domain name for the certificate.             | `tailyew.com`          |
| `EMAIL`       | Contact email for Certbot notifications.         | `admin@tailyew.com`    |
| `ACM_REGION`  | AWS region for ACM operations.                   | `us-east-1`                  |
| `IS_PRODUCTION` | Use production Certbot (`true`) or staging (`false`). | `true`                       |

---

## **Certbot Workflow**

1. **Certbot Execution**:
   - Certbot is run with the `certbot-dns-route53` plugin to request certificates for the domain specified in `DOMAIN_NAME`.

2. **Certificate Storage**:
   - Certificates (`fullchain.pem` and `privkey.pem`) are uploaded to the specified S3 bucket with AES-256 encryption.

3. **ACM Update**:
   - The function checks ACM for an existing certificate for the domain.
   - If found, the existing certificate is updated. Otherwise, a new certificate is created in ACM.

4. **Clean-Up**:
   - Certificates are removed from S3 after being imported into ACM.
   - Temporary files are deleted from the Lambda runtime's `/tmp` directory.

---

## **Makefile Commands**

### Build and Test
Use the `aws-sam-certbot` target to build and locally invoke the function:
```bash
make aws-sam-certbot
```

---

## **File Details**

### **app.py**

Core components:
1. **Certificate Management**:
   - Uses Certbot with the Route 53 plugin to generate certificates.
   - Uploads certificates to S3 and validates their integrity.

2. **ACM Integration**:
   - Checks for existing certificates in ACM for the specified domain.
   - Updates or creates certificates as needed.

3. **Error Handling**:
   - Retries Certbot operations up to 3 times on failure.
   - Logs detailed error messages for debugging.

### **requirements.txt**

Dependencies:
```text
boto3
certbot
certbot-dns-route53
```

Install with:
```bash
pip install -r requirements.txt
```

### **template.yaml**

Defines the `CertbotFunction` with the following properties:
- **Runtime**: `python3.9`
- **Timeout**: `900` seconds (15 minutes).
- **MemorySize**: `2048 MB`.
- **Environment Variables**: Configurable via AWS.
- **Event Trigger**: Scheduled to run every 60 days.

---

## **Local Testing**

Run the function locally using SAM CLI:
```bash
make aws-sam-certbot
```

---

## **Deployment**

Deploy with the aws-sam-deploy make cmd

## **Security and Best Practices**

- Use AWS Secrets Manager or SSM Parameter Store to store sensitive environment variables like `CERT_BUCKET` and `EMAIL`.
- Ensure the S3 bucket has proper access controls and encryption enabled.
- Regularly monitor Lambda execution logs in CloudWatch for potential issues.
