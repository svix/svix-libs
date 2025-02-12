using System.Net;
using System.Net.Http.Headers;
using System.Text;
using Newtonsoft.Json;

namespace Svix
{
    public class SvixHttpClient(string token, SvixOptions options)
    {
        readonly SvixOptions _options = options;
        readonly HttpClient _httpClient = new();
        private readonly string _token = token;
        private readonly JsonSerializerSettings patchJsonOptions = new();
        private readonly JsonSerializerSettings JsonOptions = new()
        {
            NullValueHandling = NullValueHandling.Ignore,
        };

        public ApiResponse<T> SendRequest<T>(
            HttpMethod method,
            string path,
            IDictionary<string, string>? pathParams = null,
            IDictionary<string, string>? queryParams = null,
            IDictionary<string, string>? headerParams = null,
            object? content = null
        )
        {
            var res = SendRequestAsync<T>(
                    method: method,
                    path: path,
                    pathParams: pathParams,
                    content: content,
                    queryParams: queryParams,
                    headerParams: headerParams
                )
                .GetAwaiter()
                .GetResult();
            return res;
        }

        public async Task<ApiResponse<T>> SendRequestAsync<T>(
            HttpMethod method,
            string path,
            IDictionary<string, string>? pathParams = null,
            IDictionary<string, string>? queryParams = null,
            IDictionary<string, string>? headerParams = null,
            object? content = null,
            CancellationToken cancellationToken = default
        )
        {
            byte[] randomBytes = new byte[8];
            new Random().NextBytes(randomBytes);
            ulong req_id = BitConverter.ToUInt64(randomBytes, 0);

            // In C# they don't let you send the same request twice :(
            var request = BuildRequest(
                method,
                path,
                req_id,
                pathParams,
                queryParams,
                headerParams,
                content
            );
            var response = await _httpClient.SendAsync(request, cancellationToken);
            for (var index = 0; index < _options.RetryScheduleMilliseconds.Count; index++)
            {
                if ((int)response.StatusCode < 500)
                {
                    break;
                }
                Thread.Sleep(_options.RetryScheduleMilliseconds[index]);
                HttpRequestMessage retryRequest = BuildRequest(
                    method,
                    path,
                    req_id,
                    pathParams,
                    queryParams,
                    headerParams,
                    content
                );
                retryRequest.Headers.Add("svix-retry-count", (index + 1).ToString());
                response = await _httpClient.SendAsync(retryRequest, cancellationToken);
            }
            return await FilterResponseForErrors<T>(response, cancellationToken);
        }

        async Task<ApiResponse<T>> FilterResponseForErrors<T>(
            HttpResponseMessage response,
            CancellationToken cancellationToken
        )
        {
            if (response.IsSuccessStatusCode)
            {
                if ((int)response.StatusCode == 204)
                {
                    return new ApiResponse<T>
                    {
                        Data = (T)(object)true,
                        StatusCode = response.StatusCode,
                    };
                }

                var responseContent = await response.Content.ReadAsStringAsync(cancellationToken);
                var data =
                    JsonConvert.DeserializeObject<T>(responseContent)
                    ?? throw new ApiException(
                        (int)response.StatusCode,
                        $"Failed to deserialize response body, status code: {(int)response.StatusCode}",
                        responseContent,
                        response.Headers
                    );
                return new ApiResponse<T> { Data = data, StatusCode = response.StatusCode };
            }

            throw new ApiException(
                (int)response.StatusCode,
                $"Request failed with status code {response.StatusCode}",
                await response.Content.ReadAsStringAsync(cancellationToken),
                response.Headers
            );
        }

        HttpRequestMessage BuildRequest(
            HttpMethod method,
            string path,
            ulong req_id,
            IDictionary<string, string>? pathParams = null,
            IDictionary<string, string>? queryParams = null,
            IDictionary<string, string>? headerParams = null,
            object? content = null
        )
        {
            var url = _options.BaseUrl;

            // Apply path parameters if provided
            if (pathParams != null)
            {
                var processedPath = path;
                foreach (var param in pathParams)
                {
                    processedPath = processedPath.Replace(
                        $"{{{param.Key}}}",
                        WebUtility.UrlEncode(param.Value)
                    );
                }
                url += processedPath;
            }
            else
            {
                url += path;
            }

            if (queryParams != null && queryParams.Count > 0)
            {
                url +=
                    "?"
                    + string.Join(
                        "&",
                        queryParams.Select(p =>
                            $"{WebUtility.UrlEncode(p.Key)}={WebUtility.UrlEncode(p.Value)}"
                        )
                    );
            }
            var request = new HttpRequestMessage(method, url);
            if (headerParams != null)
            {
                foreach (KeyValuePair<string, string> entry in headerParams)
                {
                    request.Headers.Add(entry.Key, entry.Value);
                }
            }

            request.Headers.Add("Authorization", $"Bearer {_token}");
            request.Headers.Add("svix-req-id", req_id.ToString());


            // For some reason our user-agent does not pass validation
            request.Headers.TryAddWithoutValidation("User-Agent", GetUserAgent());
            if (content != null)
            {
                string json_body;
                if (request.Method == HttpMethod.Patch)
                {
                    json_body = JsonConvert.SerializeObject(content, patchJsonOptions);
                }
                else
                {
                    json_body = JsonConvert.SerializeObject(content, JsonOptions);
                }
                var encoded_content = new StringContent(
                    json_body,
                    Encoding.UTF8,
                    "application/json"
                );
                request.Content = encoded_content;
            }
            return request;
        }

        public string GetUserAgent()
        {
            var versionQuad = GetType().Assembly.GetName().Version;

            if (versionQuad != null)
            {
                string versionQuadStr = versionQuad.ToString();
                string version;
                // C# adds an extra trailing zero so the version looks like this "1.56.0.0"
                // remove trailing zero for consistency with other libs
                if (versionQuadStr.EndsWith(".0") && versionQuadStr.Split('.').Length == 4)
                {
                    // Remove the last ".0"
                    version = versionQuadStr[..^2];
                }
                else
                {
                    version = versionQuadStr;
                }

                return $"svix-libs/{version}/csharp";
            }
            else
            {
                // If for some reason we are unable to access the version, don't panic with a nullptr deref
                return "svix-libs/missing-version/csharp";
            }
        }
    }

    public class ApiResponse<T>
    {
        public T Data { get; set; } = default!;
        public HttpStatusCode StatusCode { get; set; }
    }

    public class ApiException : Exception
    {
        public ApiException(int errorCode, string message)
            : base(message)
        {
            ErrorCode = errorCode;
        }

        public ApiException(
            int errorCode,
            string message,
            object? errorContent = null,
            HttpResponseHeaders? headers = null
        )
            : base(message)
        {
            ErrorCode = errorCode;
            ErrorContent = errorContent;
            Headers = headers;
        }

        public int ErrorCode { get; set; }
        public object? ErrorContent { get; private set; }
        public HttpResponseHeaders? Headers { get; private set; }
    }
}
