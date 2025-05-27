// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ApiTokenCensoredOut
    {
        [JsonProperty("censoredToken", Required = Required.Always)]
        public string CensoredToken { get; set; }

        [JsonProperty("createdAt", Required = Required.Always)]
        public DateTime CreatedAt { get; set; }

        [JsonProperty("expiresAt")]
        public DateTime? ExpiresAt { get; set; } = null;

        [JsonProperty("id", Required = Required.Always)]
        public string Id { get; set; }

        [JsonProperty("name")]
        public string? Name { get; set; } = null;

        [JsonProperty("scopes")]
        public List<string>? Scopes { get; set; } = null;

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class ApiTokenCensoredOut {\n");
            sb.Append("  CensoredToken: ").Append(CensoredToken).Append('\n');
            sb.Append("  CreatedAt: ").Append(CreatedAt).Append('\n');
            sb.Append("  ExpiresAt: ").Append(ExpiresAt).Append('\n');
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("  Name: ").Append(Name).Append('\n');
            sb.Append("  Scopes: ").Append(Scopes).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
