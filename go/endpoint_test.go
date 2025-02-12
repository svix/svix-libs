package svix_test

import (
	"encoding/json"
	"strings"
	"testing"

	"github.com/svix/svix-webhooks/go/models"
)

func TestEndpoint_Serialization(t *testing.T) {
	testCases := []struct {
		name            string
		testEndpoint    *models.EndpointIn
		wantChannels    bool
		wantFilterTypes bool
	}{
		{
			name: "neither channels or filter types",
			testEndpoint: &models.EndpointIn{
				Url: "https://example.svix.com/",
			},
			wantChannels:    false,
			wantFilterTypes: false,
		},
		{
			name: "channels but not filter types",
			testEndpoint: &models.EndpointIn{
				Url:      "https://example.svix.com/",
				Channels: []string{"ch1", "ch2"},
			},
			wantChannels:    true,
			wantFilterTypes: false,
		},
		{
			name: "filter types but not channels",
			testEndpoint: &models.EndpointIn{
				Url:         "https://example.svix.com/",
				FilterTypes: []string{"et1", "et2"},
			},
			wantChannels:    false,
			wantFilterTypes: true,
		},
		{
			name: "both channels and filter types",
			testEndpoint: &models.EndpointIn{
				Url:         "https://example.svix.com/",
				Channels:    []string{"ch1", "ch2"},
				FilterTypes: []string{"et1", "et2"},
			},
			wantChannels:    true,
			wantFilterTypes: true,
		},
	}

	for _, tc := range testCases {
		b, _ := json.Marshal(tc.testEndpoint)
		s := string(b)

		gotChannels := strings.Contains(s, "channels")
		gotFilterTypes := strings.Contains(s, "filterTypes")

		if tc.wantChannels && !gotChannels {
			t.Errorf("case `%s`: expected EndpointIn to have a channels field", tc.name)
		}
		if !tc.wantChannels && gotChannels {
			t.Errorf("case `%s`: expected EndpointIn to NOT have a channels field", tc.name)
		}

		if tc.wantFilterTypes && !gotFilterTypes {
			t.Errorf("case `%s`: expected EndpointIn to have a filterTypes field", tc.name)
		}
		if !tc.wantFilterTypes && gotFilterTypes {
			t.Errorf("case `%s`: expected EndpointIn to NOT have a filterTypes field", tc.name)
		}
	}
}
