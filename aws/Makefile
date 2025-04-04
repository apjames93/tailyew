# AWS CloudFormation Constants
BACKEND_STACK_NAME ?= tailyew-fn-stack
FRONTEND_CLOUDFRONT_DISTRIBUTION_ID ?= E2JO86BVTFVNRI
FRONTEND_S3_BUCKET ?= frontend.tailyew.com
FRONTEND_STACK_NAME ?= frontend-stack
FRONTEND_TEMPLATE_FILE ?= template.frontend.yaml
INFRA_STACK_NAME ?= tailyew-infra-stack
INFRA_TEMPLATE_FILE ?= template.infra.yaml

# =======================================
# Validation Targets
# =======================================

# Validate CloudFormation Templates
validate-frontend:
	aws cloudformation validate-template --template-body file://$(FRONTEND_TEMPLATE_FILE)

validate-infra:
	aws cloudformation validate-template --template-body file://$(INFRA_TEMPLATE_FILE)

# Validate SAM Templates
sam-validate:
	sam validate

# =======================================
# SAM Build and Run Targets
# =======================================


# Build the full SAM application, including all Lambda functions and layers.
# This sets the required environment variables so `pq-sys` can link against the local libpq shared library.
# The paths are resolved to absolute paths to ensure the build works regardless of working directory.
sam-build:
	@echo "🔧 Building with libpq from project-local layer..."

	RUST_LOG=debug \
	sam build $(FUNC) --beta-features


# Run Certbot Function
sam-certbot:
	sam build CertbotFunction && \
	sam local invoke CertbotFunction \
		--env-vars env.json \
		--debug

# =======================================
# Deployment Targets
# =======================================

# Deploy Infrastructure Stack
deploy-infra:
	@echo "Validating CloudFormation $(INFRA_TEMPLATE_FILE)..."
	@if aws cloudformation validate-template --template-body file://$(INFRA_TEMPLATE_FILE) > /dev/null 2>&1; then \
		echo "Template validation succeeded"; \
		aws cloudformation deploy \
			--stack-name $(INFRA_STACK_NAME) \
			--template-file $(INFRA_TEMPLATE_FILE) \
			--capabilities CAPABILITY_IAM CAPABILITY_NAMED_IAM; \
	else \
		echo "Template validation failed"; \
		exit 1; \
	fi

# Deploy SAM Stack
deploy-sam:
	@echo "Validating SAM template..."
	sam validate

	@echo "Building SAM project..."
	make sam-build || { echo "Build failed"; exit 1; }

	@echo "Deploying SAM stack: $(BACKEND_STACK_NAME)..."
	sam deploy --stack-name $(BACKEND_STACK_NAME) \
		--force-upload \
		--no-confirm-changeset --no-fail-on-empty-changeset \
		--capabilities CAPABILITY_IAM CAPABILITY_NAMED_IAM \
		--resolve-s3 \
		--debug || { echo "Deployment failed."; exit 1; }
	@echo "Deployment completed successfully for stack: $(BACKEND_STACK_NAME)."

# Deploy Frontend Stack
deploy-frontend:
	@echo "Validating CloudFormation $(FRONTEND_TEMPLATE_FILE)..."
	@if aws cloudformation validate-template --template-body file://$(FRONTEND_TEMPLATE_FILE) > /dev/null 2>&1; then \
		echo "Template validation succeeded"; \
		aws cloudformation deploy \
			--debug \
			--template-file $(FRONTEND_TEMPLATE_FILE) \
			--stack-name $(FRONTEND_STACK_NAME) \
			--capabilities CAPABILITY_IAM \
			--disable-rollback; \
	else \
		echo "Template validation failed"; \
		exit 1; \
	fi

# Upload Frontend Files to S3
upload-frontend:
	(cd .. && ENV_FILE=".env.production" make fe-build)

	# Upload all frontend files to the S3 bucket
	aws s3 sync ../frontend/static s3://$(FRONTEND_S3_BUCKET)
	aws cloudfront create-invalidation --distribution-id $(FRONTEND_CLOUDFRONT_DISTRIBUTION_ID) --paths "/*"

# Deploy Full Application
deploy-app: upload-frontend deploy-sam

# =======================================
# Deletion Targets
# =======================================

# Delete Infrastructure Stack
delete-infra:
	aws cloudformation delete-stack --stack-name $(INFRA_STACK_NAME)

# Delete SAM Stack
delete-sam:
	sam delete --no-prompts --stack-name $(BACKEND_STACK_NAME)

# Delete Frontend Stack
delete-frontend:
	aws cloudformation delete-stack --stack-name $(FRONTEND_STACK_NAME)
