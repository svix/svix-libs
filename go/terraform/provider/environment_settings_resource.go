package provider

import (
	"context"
	"github.com/hashicorp/terraform-plugin-sdk/v2/diag"
	"github.com/hashicorp/terraform-plugin-sdk/v2/helper/schema"
	"github.com/hashicorp/terraform-plugin-sdk/v2/helper/validation"
	svix "github.com/svix/svix-webhooks/go"
	"regexp"
)

func ResourceEnvironmentSettings() *schema.Resource {
	return &schema.Resource{
		CreateContext: resourceEnvironmentSettingsCreate,
		ReadContext:   resourceEnvironmentSettingsRead,
		UpdateContext: resourceEnvironmentSettingsUpdate,
		DeleteContext: resourceEnvironmentSettingsDelete,

		Schema: map[string]*schema.Schema{
			"organization_id": {
				Type:     schema.TypeString,
				Required: true,
				ForceNew: true,
			},
			"custom_color": {
				Type:     schema.TypeString,
				Optional: true,
			},
			"custom_logo_url": {
				Type:     schema.TypeString,
				Optional: true,
			},
			"custom_theme_override": {
				Type:     schema.TypeString,
				Optional: true,
			},
			"custom_strings_override": {
				Type:     schema.TypeString,
				Optional: true,
			},
			"custom_base_font_size": {
				Type:     schema.TypeInt,
				Optional: true,
			},
			"custom_font_family": {
				Type:     schema.TypeString,
				Optional: true,
				ValidateFunc: validation.StringMatch(
					regexp.MustCompile(`^[a-zA-Z0-9\-_ ]+$`),
					"must contain only alphanumeric characters, hyphens, underscores and spaces",
				),
			},
			"custom_font_family_url": {
				Type:     schema.TypeString,
				Optional: true,
			},
			"disable_endpoint_on_failure": {
				Type:     schema.TypeBool,
				Optional: true,
				Default:  true,
			},
			"display_name": {
				Type:     schema.TypeString,
				Optional: true,
			},
			"event_catalog_published": {
				Type:     schema.TypeBool,
				Optional: true,
				Default:  false,
			},
			"enforce_https": {
				Type:     schema.TypeBool,
				Optional: true,
				Default:  true,
			},
			"enable_channels": {
				Type:     schema.TypeBool,
				Optional: true,
				Default:  false,
			},
			"enable_message_tags": {
				Type:     schema.TypeBool,
				Optional: true,
				Default:  false,
			},
			"read_only": {
				Type:     schema.TypeBool,
				Optional: true,
				Default:  false,
			},
			"enable_integration_management": {
				Type:     schema.TypeBool,
				Optional: true,
				Default:  false,
			},
			"enable_transformations": {
				Type:     schema.TypeBool,
				Optional: true,
				Default:  false,
			},
			"require_endpoint_filter_types": {
				Type:     schema.TypeBool,
				Optional: true,
				Default:  false,
			},
			"require_endpoint_channel": {
				Type:     schema.TypeBool,
				Optional: true,
				Default:  false,
			},
			"color_palette_light": {
				Type:     schema.TypeString,
				Optional: true,
			},
			"color_palette_dark": {
				Type:     schema.TypeString,
				Optional: true,
			},
			"whitelabel_headers": {
				Type:     schema.TypeBool,
				Optional: true,
				Default:  false,
			},
			"show_use_svix_play": {
				Type:     schema.TypeBool,
				Optional: true,
				Default:  true,
			},
			"wipe_successful_payload": {
				Type:     schema.TypeBool,
				Optional: true,
				Default:  false,
			},
		},
	}
}

func resourceEnvironmentSettingsCreate(ctx context.Context, d *schema.ResourceData, m interface{}) diag.Diagnostics {
	client := m.(*apiClient).client

	settings := map[string]interface{}{
		"customColor":                 d.Get("custom_color").(string),
		"customLogoUrl":               d.Get("custom_logo_url").(string),
		"customThemeOverride":         d.Get("custom_theme_override").(string),
		"customStringsOverride":       d.Get("custom_strings_override").(string),
		"customBaseFontSize":          d.Get("custom_base_font_size").(int),
		"customFontFamily":            d.Get("custom_font_family").(string),
		"customFontFamilyUrl":         d.Get("custom_font_family_url").(string),
		"disableEndpointOnFailure":    d.Get("disable_endpoint_on_failure").(bool),
		"displayName":                 d.Get("display_name").(string),
		"eventCatalogPublished":       d.Get("event_catalog_published").(bool),
		"enforceHttps":                d.Get("enforce_https").(bool),
		"enableChannels":              d.Get("enable_channels").(bool),
		"enableMessageTags":           d.Get("enable_message_tags").(bool),
		"readOnly":                    d.Get("read_only").(bool),
		"enableIntegrationManagement": d.Get("enable_integration_management").(bool),
		"enableTransformations":       d.Get("enable_transformations").(bool),
		"requireEndpointFilterTypes":  d.Get("require_endpoint_filter_types").(bool),
		"requireEndpointChannel":      d.Get("require_endpoint_channel").(bool),
		"colorPaletteLight":           d.Get("color_palette_light").(string),
		"colorPaletteDark":            d.Get("color_palette_dark").(string),
		"whitelabelHeaders":           d.Get("whitelabel_headers").(bool),
		"showUseSvixPlay":             d.Get("show_use_svix_play").(bool),
		"wipeSuccessfulPayload":       d.Get("wipe_successful_payload").(bool),
	}

	_, err := client.Environment.Update(ctx, &svix.EnvironmentIn{
		Settings: settings,
	})
	if err != nil {
		return diag.FromErr(err)
	}

	d.SetId(d.Get("organization_id").(string))
	return resourceEnvironmentSettingsRead(ctx, d, m)
}

func resourceEnvironmentSettingsRead(ctx context.Context, d *schema.ResourceData, m interface{}) diag.Diagnostics {
	// Since we can't read back the settings, we'll just return nil
	return nil
}

func resourceEnvironmentSettingsUpdate(ctx context.Context, d *schema.ResourceData, m interface{}) diag.Diagnostics {
	// Update is the same as create for this resource
	return resourceEnvironmentSettingsCreate(ctx, d, m)
}

func resourceEnvironmentSettingsDelete(ctx context.Context, d *schema.ResourceData, m interface{}) diag.Diagnostics {
	// Environment settings can't be deleted, just remove from state
	d.SetId("")
	return nil
}
