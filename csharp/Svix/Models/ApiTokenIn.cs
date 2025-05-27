// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ApiTokenIn
    {
        [JsonProperty("name", Required = Required.Always)]
        public string Name { get; set; }

        [JsonProperty("scopes")]
        public List<string>? Scopes { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class ApiTokenIn {\n");
            sb.Append("  Name: ").Append(Name).Append('\n');
            sb.Append("  Scopes: ").Append(Scopes).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
