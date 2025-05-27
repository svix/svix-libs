// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EndpointStats
    {
        [JsonProperty("fail", Required = Required.Always)]
        public long Fail { get; set; }

        [JsonProperty("pending", Required = Required.Always)]
        public long Pending { get; set; }

        [JsonProperty("sending", Required = Required.Always)]
        public long Sending { get; set; }

        [JsonProperty("success", Required = Required.Always)]
        public long Success { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EndpointStats {\n");
            sb.Append("  Fail: ").Append(Fail).Append('\n');
            sb.Append("  Pending: ").Append(Pending).Append('\n');
            sb.Append("  Sending: ").Append(Sending).Append('\n');
            sb.Append("  Success: ").Append(Success).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
