// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Sent when an endpoint has been enabled.
    /// <summary>
    public class EndpointEnabledEventData
    {
        [JsonProperty("appId", Required = Required.Always)]
        public string AppId { get; set; }

        [JsonProperty("appUid")]
        public string? AppUid { get; set; } = null;

        [JsonProperty("endpointId", Required = Required.Always)]
        public string EndpointId { get; set; }

        [JsonProperty("endpointUid")]
        public string? EndpointUid { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EndpointEnabledEventData {\n");
            sb.Append("  AppId: ").Append(AppId).Append('\n');
            sb.Append("  AppUid: ").Append(AppUid).Append('\n');
            sb.Append("  EndpointId: ").Append(EndpointId).Append('\n');
            sb.Append("  EndpointUid: ").Append(EndpointUid).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
