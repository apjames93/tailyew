## 🚀 Infra Project Setup

This guide walks through provisioning infrastructure for a new TailYew project using AWS (S3 + CloudFront + ACM), including cert management via Lambda + Certbot.

---

### ✅ Prerequisites

- You have [AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/install-cliv2.html) configured with appropriate permissions
- You have `make` installed
- You own a domain via **Route 53** or have one pointed there
- Your `.env` and `aws/env.json` are configured

---

### 🏗️ Step-by-Step: New Project Provisioning

#### 1. 📦 Buy Domain in Route 53 (or connect an existing one)

- Go to [Route 53 Domains](https://console.aws.amazon.com/route53/domains)
- Buy your domain (e.g. `tailyew.com`)
- Wait for it to be active
- Grab the **Hosted Zone ID** from Route 53

#### 2. 🛠️ Update Infrastructure Templates

Edit the following files:

```bash
aws/template.infra.yaml
aws/template.frontend.yaml
```

Update:
- `HostedZoneId` parameter
- Any `DomainName`, `BucketName`, or alias values

---

#### 3. 🚀 Deploy Infra Stack

```bash
make deploy-infra
```

Creates:
- Public infra bucket (`infra.tailyew.com`)
- Route 53 A record for it
- SSM parameter for cert ARN (will be set later)

---

#### 4. 🔐 Run Certbot (Staging Mode)

Temporarily update `aws/env.json`:

```json
"IS_PRODUCTION": false
```

Then:

```bash
make sam-certbot
```

This will:
- Request a cert via Let's Encrypt staging
- Upload it to ACM (via Lambda)
- Store the ACM ARN in `/infra/acm_certificate_arn`

✅ If this succeeds, move to the next step.

---

#### 5. 🔐 Run Certbot (Production Mode)

Set `IS_PRODUCTION=true` in `aws/env.json`, then run:

```bash
make sam-certbot
```

This replaces the staging cert with a real one in ACM.

---

#### 6. 🔁 Deploy Certbot Rotation Lambda

```bash
make deploy-sam
```

This sets up a scheduled Lambda to automatically renew your certificate every 60 days.

---

#### 7. 🌍 Deploy Frontend Stack

```bash
make deploy-frontend
```

Creates:
- Private S3 bucket (`frontend.tailyew.com`)
- CloudFront distribution with custom domain + SSL
- Route 53 A record for the frontend

---

#### 8. 🖼️ Update Makefile with CloudFront ID

Once `deploy-frontend` finishes, grab the CloudFront Distribution ID and update your Makefile:

```make
FRONTEND_CLOUDFRONT_DISTRIBUTION_ID=EXXXXXXXX
```

---

#### 9. 📤 Upload Frontend Assets

```bash
make upload-frontend
```

Syncs your built Yew app to the private S3 bucket + invalidates CloudFront.

---

### ✅ Done!

Your TailYew project is now:

- 🔒 Secured with SSL (ACM + Let's Encrypt)
- 🚀 Served with low-latency via CloudFront
- 🔁 Auto-renewing certificates via Lambda
