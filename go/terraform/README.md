# Terraform Provider

This Terraform provider allows you to manage organization settings, event types, and operational webhooks.

## Installation

```hcl
terraform {
  required_providers {
    svix = {
      source = "svix/svix"
      version = "~> 1.0.0"
    }
  }
}

provider "svix" {
  api_token = "your_api_token"
  server_url = "https://api.example.com" # Optional
}
```

## Resources

### Environment Settings

```hcl
resource "svix_environment_settings" "main" {
  organization_id = "org_123"
  
  custom_color = "#FF5733"
  custom_logo_url = "https://example.com/logo.png"
  display_name = "My Organization"
  
  # Feature flags
  enable_channels = true
  enable_message_tags = true
  enable_transformations = true
  
  # Security settings
  enforce_https = true
  disable_endpoint_on_failure = true
  
  # Customization
  custom_font_family = "Open Sans"
  custom_font_family_url = "https://fonts.googleapis.com/css2?family=Open+Sans"
  custom_base_font_size = 16
  
  # Additional settings
  event_catalog_published = false
  read_only = false
  enable_integration_management = false
  require_endpoint_filter_types = false
  require_endpoint_channel = false
  whitelabel_headers = false
  show_use_svix_play = true
  wipe_successful_payload = false
}
```

### Event Types (OpenAPI Import)

```hcl
# Import from a local file
resource "svix_event_type" "from_file" {
  openapi_spec = file("${path.module}/specs/webhook.json")
  replace_all  = true  # Optional: replaces all existing event types
}

# Direct specification
resource "svix_event_type" "direct" {
  openapi_spec = jsonencode({
    info = {
      title   = "Pet Store Webhooks"
      version = "1.0.0"
    }
    openapi  = "3.1.0"
    webhooks = {
      "pet.new" = {
        post = {
          requestBody = {
            content = {
              "application/json" = {
                schema = {
                  properties = {
                    id = {
                      type   = "integer"
                      format = "int64"
                    }
                    name = {
                      type = "string"
                    }
                  }
                  required = ["id", "name"]
                }
              }
            }
          }
        }
      }
    }
  })
}
```

## Development

To build the provider locally:

```bash
go build -o terraform-provider-svix
```

To test the provider locally:

1. Build the provider:
```bash
go build -o terraform-provider-svix
```

2. Create a local provider configuration:
```bash
mkdir -p ~/.terraform.d/plugins/svix.com/svix/svix/1.0.0/linux_amd64
cp terraform-provider-svix ~/.terraform.d/plugins/svix.com/svix/svix/1.0.0/linux_amd64/
```

3. Create a test configuration and run terraform:
```bash
mkdir test && cd test
# Create test.tf with configuration
terraform init
terraform plan
```
```
