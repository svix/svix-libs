package svix_test

import (
	"context"
	"encoding/json"
	"errors"
	"io"
	"math/rand/v2"
	"net/http"
	"net/url"
	"os"
	"reflect"
	"sort"
	"strconv"
	"strings"
	"testing"
	"time"

	"github.com/jarcoal/httpmock"
	svix "github.com/svix/svix-webhooks/go"
	"github.com/svix/svix-webhooks/go/models"
	"github.com/svix/svix-webhooks/go/utils"
)

var endpointOurStr = `
{
  "id": "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2",
  "metadata": {
    "property1": "string",
    "property2": "string"
  },
  "description": "string",
  "rateLimit": 0,
  "uid": "unique-identifier",
  "url": "https://example.com/webhook/",
  "version": 1,
  "disabled": false,
  "filterTypes": [
    "user.signup",
    "user.deleted"
  ],
  "channels": [
    "project_123",
    "group_2"
  ],
  "createdAt": "2019-08-24T14:15:22Z",
  "updatedAt": "2019-08-24T14:15:22Z"
}`

// Builds an API client for testing against an arbitrary API server with an arbitrary token.
//
// The connection details are pulled from the environment, e.g. `SVIX_TOKEN` and `SVIX_SERVER_URL`.
// In the case that either are unset, a test that calls this function will automatically skip.
func getTestClient(t *testing.T) *svix.Svix {
	t.Helper()

	token, exists := os.LookupEnv("SVIX_TOKEN")
	if !exists {
		t.Skipf("Unable to construct test client (`SVIX_TOKEN` unset)")
		return nil
	}
	rawServerUrl, exists := os.LookupEnv("SVIX_SERVER_URL")
	if !exists {
		t.Skipf("Unable to construct test client (`SVIX_SERVER_URL` unset)")
		return nil
	}
	serverUrl, err := url.Parse(rawServerUrl)
	if err != nil {
		panic(err)
	}
	svx, err := svix.New(token, &svix.SvixOptions{
		ServerUrl: serverUrl,
	})
	if err != nil {
		panic(err)
	}
	return svx
}

// Suppresses a request error response if it's a 409
func isNotConflict(err error) error {
	if err != nil {
		var svixError svix.Error
		if errors.As(err, &svixError) {
			if svixError.Status() == http.StatusConflict {
				// Pass if we see the suppressed status
				return nil
			}
		}
	}
	return err
}

// Runs through some common API interactions.
func TestKitchenSink(t *testing.T) {
	ctx := context.Background()
	client := getTestClient(t)

	app, err := client.Application.Create(ctx, models.ApplicationIn{
		Name: "test",
	}, nil)
	if err != nil {
		t.Fatal(err)
	}

	_, err = client.EventType.Create(ctx, models.EventTypeIn{Name: "event.started", Description: "Something started"}, nil)

	if isNotConflict(err) != nil {
		t.Fatal(err)
	}

	_, err = client.EventType.Create(ctx, models.EventTypeIn{Name: "event.ended", Description: "Something ended"}, nil)
	if isNotConflict(err) != nil {
		t.Fatal(err)
	}

	endp, err := client.Endpoint.Create(ctx, app.Id, models.EndpointIn{
		Url: "https://example.svix.com/",
	}, nil)
	if err != nil {
		t.Fatal(err)
	}

	endpPatch := models.EndpointPatch{
		FilterTypes: utils.NewNullable([]string{"event.started", "event.ended"}),
	}

	patched, err := client.Endpoint.Patch(ctx, app.Id, endp.Id, endpPatch)
	if err != nil {
		t.Fatal(err)
	}

	for _, typ := range patched.FilterTypes {
		if !(typ == "event.started" || typ == "event.ended") {
			t.Fatalf("unexpected filter type: `%s`", typ)
		}
	}
}

func TestStaticNullableString(t *testing.T) {
	app := &models.ApplicationPatch{
		Uid: utils.NewNullable("my-uid"),
	}

	if !app.Uid.IsSet() {
		t.Fatalf("app.Uid is not set but should be")
	}

	if *app.Uid.Get() != "my-uid" {
		t.Fatalf("app.Uid has unexpected value: `%s`", *app.Uid.Get())
	}

	app.Uid.Unset()
	if app.Uid.IsSet() {
		t.Fatalf("app.Uid is set but shouldn't be")
	}
}

func assertMarshalEq(v any, expected string, t *testing.T) {
	bytes, err := json.Marshal(v)
	if err != nil {
		t.Error("JSON serialization failed", err)
		return
	}

	s := string(bytes)
	if s != expected {
		t.Errorf("Unexpected serialization expected: `%s`  got: `%s`", expected, s)
	}
}

func TestModelSerialization(t *testing.T) {
	ep_in := models.EndpointIn{
		Url: "http://example.local",
	}
	assertMarshalEq(ep_in, `{"url":"http://example.local"}`, t)

	uid := "test"
	ep_in.Uid = &uid
	assertMarshalEq(ep_in, `{"uid":"test","url":"http://example.local"}`, t)

	uid = ""
	ep_in.Uid = &uid
	assertMarshalEq(ep_in, `{"uid":"","url":"http://example.local"}`, t)

	metadata := make(map[string]string)
	ep_in.Uid = nil
	ep_in.Metadata = &metadata
	assertMarshalEq(ep_in, `{"metadata":{},"url":"http://example.local"}`, t)

	ep_patch := models.EndpointPatch{}
	assertMarshalEq(ep_patch, `{}`, t)

	// FIXME: setting channels to `null` not currently possible
	ep_patch.Channels = utils.NewNullable([]string{})
	assertMarshalEq(ep_patch, `{"channels":[]}`, t)

	ep_patch.Channels = utils.NewNullable([]string{"foo"})
	assertMarshalEq(ep_patch, `{"channels":["foo"]}`, t)

	ep_patch.Channels = utils.NewUnsetNullable[[]string]()
	disabled := false
	ep_patch.Disabled = &disabled
	assertMarshalEq(ep_patch, `{"disabled":false}`, t)

	ep_patch.Disabled = nil
	ep_patch.Uid = utils.NewNullableFromPtr[string](nil)
	assertMarshalEq(ep_patch, `{"uid":null}`, t)
}

func TestEndpointPatchNullableListSerialization(t *testing.T) {
	// if nullable field is set, marshal should serialize that field
	ep_patch1 := models.EndpointPatch{
		Channels: utils.NewNullable([]string{"asd"}),
	}
	assertMarshalEq(ep_patch1, `{"channels":["asd"]}`, t)

	// if nullable field is explicitly set null, marshal should emit a null
	ep_patch2 := models.EndpointPatch{
		Channels: utils.NewNullableFromPtr[[]string](nil),
	}
	assertMarshalEq(ep_patch2, `{"channels":null}`, t)

	// if nullable field is omitted, marshal should omitted that field
	ep_patch3 := models.EndpointPatch{}
	assertMarshalEq(ep_patch3, `{}`, t)

}
func TestEndpointPatchNullableStringSerialization(t *testing.T) {
	// if nullable field is set, marshal should serialize that field
	ep_patch1 := models.EndpointPatch{
		Uid: utils.NewNullable("asd"),
	}
	assertMarshalEq(ep_patch1, `{"uid":"asd"}`, t)

	// if nullable field is explicitly set null, marshal should emit a null
	ep_patch2 := models.EndpointPatch{
		Uid: utils.NewNullableFromPtr[string](nil),
	}
	assertMarshalEq(ep_patch2, `{"uid":null}`, t)

	// if nullable field is omitted, marshal should omitted that field
	ep_patch3 := models.EndpointPatch{}
	assertMarshalEq(ep_patch3, `{}`, t)

}

func TestModelDeserialization(t *testing.T) {
	var ep_out models.EndpointOut

	// only required properties
	err := json.Unmarshal(
		[]byte(`{
			"id": "ep_foo",
			"metadata": {"foo": "bar"},
			"description": "x",
			"url": "http://example.local",
			"version": 1,
			"createdAt": "2019-08-24T14:15:22Z",
			"updatedAt": "2019-08-24T14:15:22Z"
		}`),
		&ep_out,
	)
	if err != nil {
		t.Fatal("JSON deserialization failed", err)
	}
	if len(ep_out.Channels) != 0 {
		t.Error("unexpected value for channels", ep_out.Channels)
	}
	if ep_out.Id != "ep_foo" {
		t.Error("unexpected value for id", ep_out.Id)
	}
	if len(ep_out.Metadata) != 1 {
		t.Error("unexpected value for metadata", ep_out.Metadata)
	}
	if ep_out.Metadata["foo"] != "bar" {
		t.Error("unexpected value for metadata", ep_out.Metadata)
	}
	if ep_out.Description != "x" {
		t.Error("unexpected value for description", ep_out.Description)
	}
	if ep_out.Disabled != nil {
		t.Error("unexpected value for disabled", ep_out.Disabled)
	}
	if len(ep_out.FilterTypes) != 0 {
		t.Error("unexpected value for filterTypes", ep_out.FilterTypes)
	}
	if ep_out.RateLimit != nil {
		t.Error("unexpected value for rateLimit", ep_out.RateLimit)
	}
	if ep_out.Uid != nil {
		t.Error("unexpected value for uid", ep_out.Uid)
	}
	if ep_out.Url != "http://example.local" {
		t.Error("unexpected value for url", ep_out.Url)
	}
	if ep_out.Version != 1 {
		t.Error("unexpected value for version", ep_out.Version)
	}

	ep_out = models.EndpointOut{}
	// full example from API docs
	err = json.Unmarshal(
		[]byte(`{
			"id": "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2",
			"metadata": {
				"property1": "string",
				"property2": "string"
			},
			"description": "string",
			"rateLimit": 0,
			"uid": "unique-ep-identifier",
			"url": "https://example.com/webhook/",
			"version": 1,
			"disabled": false,
			"filterTypes": [
				"user.signup",
				"user.deleted"
			],
			"channels": [
				"project_123",
				"group_2"
			],
			"createdAt": "2019-08-24T14:15:22Z",
			"updatedAt": "2019-08-24T14:15:22Z"
		}`),
		&ep_out,
	)
	if err != nil {
		t.Fatal("JSON deserialization failed", err)
	}
	if ep_out.Id != "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2" {
		t.Error("unexpected value for id", ep_out.Id)
	}
	if len(ep_out.Metadata) != 2 {
		t.Error("unexpected value for metadata", ep_out.Metadata)
	}
	if ep_out.Metadata["property1"] != "string" {
		t.Error("unexpected value for metadata", ep_out.Metadata)
	}
	if ep_out.Metadata["property2"] != "string" {
		t.Error("unexpected value for metadata", ep_out.Metadata)
	}
	if ep_out.Description != "string" {
		t.Error("unexpected value for description", ep_out.Description)
	}
	if *ep_out.RateLimit != 0 {
		t.Error("unexpected value for rateLimit", ep_out.RateLimit)
	}
	if *ep_out.Uid != "unique-ep-identifier" {
		t.Error("unexpected value for uid", ep_out.Uid)
	}
	if ep_out.Url != "https://example.com/webhook/" {
		t.Error("unexpected value for url", ep_out.Url)
	}
	if ep_out.Version != 1 {
		t.Error("unexpected value for version", ep_out.Version)
	}
	if *ep_out.Disabled != false {
		t.Error("unexpected value for disabled", ep_out.Disabled)
	}
	if len(ep_out.FilterTypes) != 2 {
		t.Error("unexpected value for filterTypes", ep_out.FilterTypes)
	}
	if ep_out.FilterTypes[0] != "user.signup" {
		t.Error("unexpected value for filterTypes", ep_out.FilterTypes)
	}
	if ep_out.FilterTypes[1] != "user.deleted" {
		t.Error("unexpected value for filterTypes", ep_out.FilterTypes)
	}
	if len(ep_out.Channels) != 2 {
		t.Error("unexpected value for channels", ep_out.Channels)
	}
	if ep_out.Channels[0] != "project_123" {
		t.Error("unexpected value for channels", ep_out.Channels)
	}
	if ep_out.Channels[1] != "group_2" {
		t.Error("unexpected value for channels", ep_out.Channels)
	}

	ep_out = models.EndpointOut{}
	// same example with optional / nullable fields set to null explicitly
	// (and empty metadata)
	err = json.Unmarshal(
		[]byte(`{
			"id": "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2",
			"metadata": {},
			"description": "string",
			"rateLimit": null,
			"uid": null,
			"url": "https://example.com/webhook/",
			"version": 1,
			"disabled": null,
			"filterTypes": null,
			"channels": null,
			"createdAt": "2019-08-24T14:15:22Z",
			"updatedAt": "2019-08-24T14:15:22Z"
		}`),
		&ep_out,
	)
	if err != nil {
		t.Fatal("JSON deserialization failed", err)
	}
	if ep_out.Id != "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2" {
		t.Error("unexpected value for id", ep_out.Id)
	}
	if len(ep_out.Metadata) != 0 {
		t.Error("unexpected value for metadata", ep_out.Metadata)
	}
	if ep_out.Description != "string" {
		t.Error("unexpected value for description", ep_out.Description)
	}
	if ep_out.RateLimit != nil {
		t.Error("unexpected value for rateLimit", ep_out.RateLimit)
	}
	if ep_out.Uid != nil {
		t.Error("unexpected value for uid", ep_out.Uid)
	}
	if ep_out.Url != "https://example.com/webhook/" {
		t.Error("unexpected value for url", ep_out.Url)
	}
	if ep_out.Version != 1 {
		t.Error("unexpected value for version", ep_out.Version)
	}
	if ep_out.Disabled != nil {
		t.Error("unexpected value for disabled", ep_out.Disabled)
	}
	if len(ep_out.FilterTypes) != 0 {
		t.Error("unexpected value for filterTypes", ep_out.FilterTypes)
	}
	if len(ep_out.Channels) != 0 {
		t.Error("unexpected value for channels", ep_out.Channels)
	}
}

func TestUTCTimeSerialization(t *testing.T) {
	timeSince := time.Date(2025, 2, 11, 9, 12, 42, 514420438, time.UTC)
	timeUntil := time.Date(1980, 12, 30, 18, 12, 42, 438420514, time.UTC)

	appUsage := models.AppUsageStatsIn{
		Since: timeSince,
		Until: timeUntil,
	}
	assertMarshalEq(appUsage, `{"since":"2025-02-11T09:12:42.514420438Z","until":"1980-12-30T18:12:42.438420514Z"}`, t)
}

func TestUTCTimeDeserialization(t *testing.T) {
	timeSince := time.Date(2025, 2, 11, 9, 12, 42, 514420438, time.UTC)
	timeUntil := time.Date(1980, 12, 30, 18, 12, 42, 438420514, time.UTC)

	appUsageJson := `{"since":"2025-02-11T09:12:42.514420438Z","until":"1980-12-30T18:12:42.438420514Z"}`
	var appUsage models.AppUsageStatsIn
	err := json.Unmarshal([]byte(appUsageJson), &appUsage)
	if err != nil {
		t.Error(err)
	}

	if appUsage.Since != timeSince {
		t.Error("unexpected value for since", appUsage.Since)
	}
	if appUsage.Until != timeUntil {
		t.Error("unexpected value for until", appUsage.Until)
	}
}

func TestApplicationPatchNullableAgainstServer(t *testing.T) {
	ctx := context.Background()
	client := getTestClient(t)
	origUid := strconv.FormatUint(rand.Uint64(), 10)
	app, err := client.Application.Create(ctx, models.ApplicationIn{Name: "test app", Metadata: &map[string]string{"key1": "old val1", "key3": "untouched"}, Uid: &origUid}, nil)
	if err != nil {
		t.Fatal(err)
	}

	// uid is unset
	appPatch1 := models.ApplicationPatch{
		Metadata: &map[string]string{
			"key1": "val1",
			"key2": "val2",
		},
	}
	patchRes1, err := client.Application.Patch(ctx, app.Id, appPatch1)
	if err != nil {
		t.Fatal(err)
	}
	if patchRes1.Uid == nil {
		t.Errorf("Unexpected Uid %v", patchRes1)
	}
	if *patchRes1.Uid != origUid {
		t.Errorf("Unexpected Uid %v", patchRes1)
	}

	// uid is explicitly set to null
	appPatch2 := models.ApplicationPatch{
		Uid: utils.NewNullableFromPtr[string](nil),
	}
	patchRes2, err := client.Application.Patch(ctx, app.Id, appPatch2)
	if err != nil {
		t.Fatal(err)
	}
	if patchRes2.Uid != nil {
		t.Errorf("Unexpected, Uid should be null %v", patchRes2)
	}

	// new uid to override server uid
	newUid := strconv.FormatUint(rand.Uint64(), 10)
	appPatch3 := models.ApplicationPatch{
		Uid: utils.NewNullable(newUid),
	}
	patchRes3, err := client.Application.Patch(ctx, app.Id, appPatch3)
	if err != nil {
		t.Fatal(err)
	}
	if patchRes3.Uid == nil {
		t.Errorf("Unexpected Uid %v", patchRes3)
	}
	if *patchRes3.Uid != newUid {
		t.Errorf("Unexpected Uid %v", patchRes3)
	}
}

func TestEndpointPatchNullableAgainstServer(t *testing.T) {
	ctx := context.Background()
	client := getTestClient(t)
	app, err := client.Application.Create(ctx, models.ApplicationIn{Name: "test app", Metadata: &map[string]string{"key1": "old val1", "key3": "untouched"}}, nil)
	if err != nil {
		t.Fatal(err)
	}
	endp, err := client.Endpoint.Create(ctx, app.Id, models.EndpointIn{
		Url: "https://play.svix.com",
	}, nil)
	if err != nil {
		t.Fatal(err)
	}
	if endp.Channels != nil {
		t.Errorf("Unexpected Channels %v", endp.Channels)
	}

	// Patching channels should send the list to the server
	patch1 := models.EndpointPatch{
		Channels: utils.NewNullable([]string{"non-sorted-text", "ch2", "ch7"}),
	}
	endp2, err := client.Endpoint.Patch(ctx, app.Id, endp.Id, patch1)
	if err != nil {
		t.Fatal(err)
	}

	sort.Slice(endp2.Channels, func(i, j int) bool { return strings.ToLower(endp2.Channels[i]) < strings.ToLower(endp2.Channels[j]) })
	if !reflect.DeepEqual(endp2.Channels, []string{"ch2", "ch7", "non-sorted-text"}) {
		t.Errorf("Unexpected  Channels %v", endp2.Channels)
	}

	// Patching channels with nil should send a null to the server
	patch2 := models.EndpointPatch{
		Channels: utils.NewNullable[[]string](nil),
	}
	endp3, err := client.Endpoint.Patch(ctx, app.Id, endp.Id, patch2)
	if err != nil {
		t.Fatal(err)
	}
	// an empty slice becomes nil
	if endp3.Channels != nil {
		t.Errorf("Unexpected Channels %v", endp3.Channels)
	}
}

func TestEndpointPatchSerialization(t *testing.T) {
	ctx := context.Background()
	svx := newMockClient()
	httpmock.Activate()
	defer httpmock.DeactivateAndReset()

	httpmock.RegisterResponder("PATCH", "http://testapi.test/api/v1/app/app1/endpoint/endp1",
		func(r *http.Request) (*http.Response, error) {
			defer r.Body.Close()
			body, err := io.ReadAll(r.Body)
			if err != nil {
				t.Error(err)
			}
			bodyStr := string(body)
			if bodyStr != `{"channels":null}` {
				t.Errorf("Unexpected body %s", bodyStr)
			}
			return httpmock.NewStringResponse(200, endpointOurStr), nil
		},
	)
	patch := models.EndpointPatch{
		Channels: utils.NewNullable[[]string](nil),
	}

	_, err := svx.Endpoint.Patch(ctx, "app1", "endp1", patch)
	if err != nil {
		t.Error(err)
	}
}

func TestEndpointPatchUnsetNotSentToServer(t *testing.T) {
	ctx := context.Background()
	svx := newMockClient()
	httpmock.Activate()
	defer httpmock.DeactivateAndReset()

	httpmock.RegisterResponder("PATCH", "http://testapi.test/api/v1/app/app1/endpoint/endp1",
		func(r *http.Request) (*http.Response, error) {
			defer r.Body.Close()
			body, err := io.ReadAll(r.Body)
			if err != nil {
				t.Error(err)
			}
			bodyStr := string(body)
			if bodyStr != `{"uid":"here so a body is sent"}` {
				t.Errorf("Unexpected body %s", bodyStr)
			}
			return httpmock.NewStringResponse(200, endpointOurStr), nil
		},
	)
	patch := models.EndpointPatch{
		Uid: utils.NewNullable("here so a body is sent"),
		// this will not be sent to server
		Channels: utils.NewUnsetNullable[[]string](),
	}

	_, err := svx.Endpoint.Patch(ctx, "app1", "endp1", patch)
	if err != nil {
		t.Error(err)
	}
}
