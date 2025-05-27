// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class PollingEndpointConsumerSeekIn
    {
        [JsonProperty("after", Required = Required.Always)]
        public DateTime After { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class PollingEndpointConsumerSeekIn {\n");
            sb.Append("  After: ").Append(After).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
