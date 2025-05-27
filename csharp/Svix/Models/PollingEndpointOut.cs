// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class PollingEndpointOut
    {
        [JsonProperty("data", Required = Required.Always)]
        public List<PollingEndpointMessageOut> Data { get; set; }

        [JsonProperty("done", Required = Required.Always)]
        public bool Done { get; set; }

        [JsonProperty("iterator", Required = Required.Always)]
        public string Iterator { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class PollingEndpointOut {\n");
            sb.Append("  Data: ").Append(Data).Append('\n');
            sb.Append("  Done: ").Append(Done).Append('\n');
            sb.Append("  Iterator: ").Append(Iterator).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
