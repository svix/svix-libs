// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class PollingEndpointConsumerSeekOut
    {
        [JsonProperty("iterator", Required = Required.Always)]
        public string Iterator { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class PollingEndpointConsumerSeekOut {\n");
            sb.Append("  Iterator: ").Append(Iterator).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
