// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class AdobeSignConfig
    {
        [JsonProperty("clientId", Required = Required.Always)]
        public string ClientId { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class AdobeSignConfig {\n");
            sb.Append("  ClientId: ").Append(ClientId).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
