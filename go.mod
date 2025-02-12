module github.com/svix/svix-webhooks

go 1.20

retract (
	v1.55.0
	v1.54.0
	// v1.53.0 was never published
	v1.52.0
	v1.51.0
	v1.50.0
	v1.49.0
	v1.48.0
	v1.47.0
	v1.46.0
)

require gopkg.in/validator.v2 v2.0.1

require github.com/jarcoal/httpmock v1.3.1 // indirect
