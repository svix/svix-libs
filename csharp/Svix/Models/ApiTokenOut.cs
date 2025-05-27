// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class ApiTokenOut
    {
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

        [JsonProperty("token", Required = Required.Always)]
        public string Token { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class ApiTokenOut {\n");
            sb.Append("  CreatedAt: ").Append(CreatedAt).Append('\n');
            sb.Append("  ExpiresAt: ").Append(ExpiresAt).Append('\n');
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("  Name: ").Append(Name).Append('\n');
            sb.Append("  Scopes: ").Append(Scopes).Append('\n');
            sb.Append("  Token: ").Append(Token).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
