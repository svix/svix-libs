// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class CronConfig
    {
        [JsonProperty("contentType")]
        public string? ContentType { get; set; } = null;

        [JsonProperty("payload", Required = Required.Always)]
        public string Payload { get; set; }

        [JsonProperty("schedule", Required = Required.Always)]
        public string Schedule { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class CronConfig {\n");
            sb.Append("  ContentType: ").Append(ContentType).Append('\n');
            sb.Append("  Payload: ").Append(Payload).Append('\n');
            sb.Append("  Schedule: ").Append(Schedule).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
