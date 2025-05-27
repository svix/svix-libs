// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class IngestEndpointSecretOut
    {
        [JsonProperty("key", Required = Required.Always)]
        public string Key { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class IngestEndpointSecretOut {\n");
            sb.Append("  Key: ").Append(Key).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
