package provider

import (
	"context"
	"github.com/hashicorp/terraform-plugin-sdk/v2/diag"
	"github.com/hashicorp/terraform-plugin-sdk/v2/helper/schema"
	svix "github.com/svix/svix-webhooks/go"
)

type apiClient struct {
	client *svix.Svix
}

func Provider() *schema.Provider {
	return &schema.Provider{
		ResourcesMap: map[string]*schema.Resource{
			"svix_environment_settings": ResourceEnvironmentSettings(),
			"svix_event_type":           ResourceEventType(),
		},
		DataSourcesMap: map[string]*schema.Resource{
			// Data sources will be added here
		},
		Schema: map[string]*schema.Schema{
			"api_token": {
				Type:      schema.TypeString,
				Required:  true,
				Sensitive: true,
			},
			"server_url": {
				Type:     schema.TypeString,
				Optional: true,
			},
		},
		ConfigureContextFunc: providerConfigure,
	}
}

func providerConfigure(ctx context.Context, d *schema.ResourceData) (interface{}, diag.Diagnostics) {
	token := d.Get("api_token").(string)
	serverUrl := d.Get("server_url").(string)

	var options *svix.Options
	if serverUrl != "" {
		options = &svix.Options{
			ServerURL: serverUrl,
		}
	}

	client := svix.New(token, options)
	return &apiClient{client: client}, nil
}
