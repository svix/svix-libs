// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    /// <summary>
    /// Sent when an endpoint is updated.
    /// <summary>
    public class EndpointUpdatedEvent
    {
        [JsonProperty("data", Required = Required.Always)]
        public EndpointUpdatedEventData Data { get; set; }

        [JsonProperty("type", Required = Required.Always)]
        public string Type { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EndpointUpdatedEvent {\n");
            sb.Append("  Data: ").Append(Data).Append('\n');
            sb.Append("  Type: ").Append(Type).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
