// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class AppPortalAccessOut
    {
        [JsonProperty("token", Required = Required.Always)]
        public string Token { get; set; }

        [JsonProperty("url", Required = Required.Always)]
        public string Url { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class AppPortalAccessOut {\n");
            sb.Append("  Token: ").Append(Token).Append('\n');
            sb.Append("  Url: ").Append(Url).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
