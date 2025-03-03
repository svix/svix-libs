package provider

import (
	"context"
	"github.com/hashicorp/terraform-plugin-sdk/v2/diag"
	"github.com/hashicorp/terraform-plugin-sdk/v2/helper/schema"
)

func ResourceEventType() *schema.Resource {
	return &schema.Resource{
		CreateContext: resourceEventTypeCreate,
		ReadContext:   resourceEventTypeRead,
		UpdateContext: resourceEventTypeUpdate,
		DeleteContext: resourceEventTypeDelete,
		Schema: map[string]*schema.Schema{
			"openapi_spec": {
				Type:        schema.TypeString,
				Required:    true,
				Description: "OpenAPI specification as a JSON string or loaded from a file using file() function",
			},
			"replace_all": {
				Type:        schema.TypeBool,
				Optional:    true,
				Default:     false,
				Description: "If true, replaces all existing event types with the imported ones",
			},
		},
	}
}

func resourceEventTypeCreate(ctx context.Context, d *schema.ResourceData, m interface{}) diag.Diagnostics {
	client := m.(*apiClient)

	spec := d.Get("openapi_spec").(string)

	out, err := client.EventType.ImportOpenapi(ctx, &EventTypeImportOpenApiIn{
		SpecRaw:    spec,
		ReplaceAll: d.Get("replace_all").(bool),
	})
	if err != nil {
		return diag.FromErr(err)
	}

	// Set the ID to the first event type created
	// Note: You might want to adjust this based on your actual response structure
	if len(out.EventTypes) > 0 {
		d.SetId(out.EventTypes[0].ID)
	}

	return resourceEventTypeRead(ctx, d, m)
}

func resourceEventTypeRead(ctx context.Context, d *schema.ResourceData, m interface{}) diag.Diagnostics {
	client := m.(*apiClient)
	// Implement read logic using your SDK
	return diag.Diagnostics{}
}

func resourceEventTypeUpdate(ctx context.Context, d *schema.ResourceData, m interface{}) diag.Diagnostics {
	client := m.(*apiClient)
	// Implement update logic using your SDK
	return diag.Diagnostics{}
}

func resourceEventTypeDelete(ctx context.Context, d *schema.ResourceData, m interface{}) diag.Diagnostics {
	client := m.(*apiClient)
	// Implement delete logic using your SDK
	return diag.Diagnostics{}
}
