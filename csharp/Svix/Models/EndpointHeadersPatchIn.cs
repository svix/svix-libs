// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class EndpointHeadersPatchIn
    {
        [JsonProperty("headers", Required = Required.Always)]
        public Dictionary<string, string> Headers { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class EndpointHeadersPatchIn {\n");
            sb.Append("  Headers: ").Append(Headers).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
