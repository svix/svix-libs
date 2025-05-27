// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Sent when an endpoint is deleted.
    /// <summary>
    public class EndpointDeletedEvent
    {
        [JsonProperty("data", Required = Required.Always)]
        public EndpointDeletedEventData Data { get; set; }

        [JsonProperty("type", Required = Required.Always)]
        public string Type { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EndpointDeletedEvent {\n");
            sb.Append("  Data: ").Append(Data).Append('\n');
            sb.Append("  Type: ").Append(Type).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
