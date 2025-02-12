package svix

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"math/rand/v2"
	"net/http"
	"net/http/httputil"
	"net/url"
	"reflect"
	"strconv"
	"strings"
	"time"
)

type SvixHttpClient struct {
	DefaultHeaders map[string]string
	HTTPClient     *http.Client
	RetrySchedule  []time.Duration
	BaseURL        string
	Debug          bool
}

func defaultSvixHttpClient() SvixHttpClient {
	return SvixHttpClient{
		DefaultHeaders: map[string]string{},
		HTTPClient:     &http.Client{Timeout: 60 * time.Second},
		RetrySchedule:  []time.Duration{50 * time.Microsecond, 100 * time.Microsecond, 200 * time.Microsecond},
		BaseURL:        "https://api.eu.svix.com",
		Debug:          false,
	}
}

func executeRequest[ReqBody any, ResBody any](
	ctx context.Context,
	client *SvixHttpClient,
	method string,
	path string,
	pathParams map[string]string,
	queryParams map[string]string,
	headerParams map[string]string,
	jsonBody *ReqBody,

) (*ResBody, error) {

	urlWithPath := client.BaseURL + replacePathKeys(path, pathParams)
	urlStr, err := addQueryParams(urlWithPath, queryParams)
	if err != nil {
		return nil, err
	}
	encodedBody, err := json.Marshal(jsonBody)
	if err != nil {
		return nil, err
	}
	req, err := http.NewRequestWithContext(ctx, method, urlStr, bytes.NewBuffer(encodedBody))
	if err != nil {
		return nil, err
	}

	req.Header.Set("svix-req-id", strconv.FormatUint(rand.Uint64(), 10))
	for hKey, hVal := range client.DefaultHeaders {
		req.Header.Add(hKey, hVal)
	}
	for hKey, hVal := range headerParams {
		req.Header.Add(hKey, hVal)
	}

	res, err := executeRequestWithRetries(client, req)

	if err != nil {
		return nil, err
	}
	if res.StatusCode == 204 {
		return nil, nil
	}
	defer res.Body.Close()

	body, err := io.ReadAll(res.Body)
	if err != nil {
		return nil, err
	}
	if res.StatusCode >= 200 && res.StatusCode <= 299 {
		var ret ResBody
		err = json.Unmarshal(body, &ret)
		if err != nil {
			return nil, err
		}

		return &ret, nil
	}
	return nil, Error{
		status: res.StatusCode,
		body:   body,
		error:  fmt.Sprintf("status code %s", res.Status),
	}

}

func executeRequestWithRetries(client *SvixHttpClient, request *http.Request) (*http.Response, error) {
	if client.Debug {
		log.Printf("URL: %s", request.URL)
		dump, err := httputil.DumpRequestOut(request, true)
		if err != nil {
			return nil, err
		}
		log.Printf("\n%s\n", string(dump))
	}

	resp, err := client.HTTPClient.Do(request)
	for try := 0; try < len(client.RetrySchedule); try++ {
		if err == nil && resp.StatusCode < 500 {
			break
		}
		request.Header.Set("svix-retry-count", strconv.Itoa(try+1))
		sleepTime := client.RetrySchedule[try]
		time.Sleep(sleepTime)
		resp, err = client.HTTPClient.Do(request)
	}

	if client.Debug {
		dump, err := httputil.DumpResponse(resp, true)
		if err != nil {
			return resp, err
		}
		log.Printf("\n%s\n", string(dump))
	}
	return resp, err
}

func replacePathKeys(path string, pathParams map[string]string) string {
	for key, value := range pathParams {
		placeholder := "{" + key + "}"
		path = strings.ReplaceAll(path, placeholder, value)
	}
	return path
}

func addQueryParams(baseURL string, params map[string]string) (string, error) {
	parsedURL, err := url.Parse(baseURL)
	if err != nil {
		return "", err
	}

	query := parsedURL.Query()
	for key, value := range params {
		query.Add(key, value)
	}

	parsedURL.RawQuery = query.Encode()
	return parsedURL.String(), nil
}

func serializeParamToMap(key string, val interface{}, d map[string]string, err *error) {
	// I pass the error in here so I don't have to "if err != nil" for every query param
	if *err != nil {
		return
	}
	// If val is null don't add it to the query params map
	if val == nil || (reflect.ValueOf(val).Kind() == reflect.Ptr && reflect.ValueOf(val).IsNil()) {
		return
	}

	v, localErr := serializeQueryOrHeaderParam(val, key)
	if localErr != nil {
		*err = localErr
	} else {
		d[key] = v
	}
}

func serializeQueryOrHeaderParam(val interface{}, key string) (string, error) {
	v := reflect.ValueOf(val)
	var value string
	if val == nil || (reflect.ValueOf(val).Kind() == reflect.Ptr && reflect.ValueOf(val).IsNil()) {
		return "", fmt.Errorf("can't serialize nil as a query param, key: %s", key)
	}

	switch v.Kind() {
	case reflect.Pointer:
		innerVal, err := serializeQueryOrHeaderParam(v.Elem().Interface(), key)
		if err != nil {
			return "", err
		}
		value = innerVal
	case reflect.String:
		value = v.String()
	case reflect.Int, reflect.Int8, reflect.Int16, reflect.Int32, reflect.Int64:
		value = strconv.FormatInt(v.Int(), 10)
	case reflect.Uint, reflect.Uint8, reflect.Uint16,
		reflect.Uint32, reflect.Uint64:
		value = strconv.FormatUint(v.Uint(), 10)
	case reflect.Float32, reflect.Float64:
		value = strconv.FormatFloat(v.Float(), 'g', -1, 64)
	case reflect.Bool:
		if v.Bool() {
			value = "true"
		} else {
			value = "false"
		}
	case reflect.Slice:
		// we are assuming that the inner type is a simple type (no nested lists)
		serializedValues := make([]string, v.Len())
		for i := 0; i < v.Len(); i++ {
			serializedVal, err := serializeQueryOrHeaderParam(v.Index(i).Interface(), key)
			if err != nil {
				return "", err
			}
			serializedValues[i] = serializedVal
		}
		value = strings.Join(serializedValues, ",")

	default:
		return "", fmt.Errorf("can't serialize %s as a query param, key: %s", v.Kind().String(), key)

	}
	return value, nil
}
