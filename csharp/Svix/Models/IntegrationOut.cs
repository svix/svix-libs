// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class IntegrationOut
    {
        [JsonProperty("createdAt", Required = Required.Always)]
        public DateTime CreatedAt { get; set; }

        [JsonProperty("featureFlags")]
        public List<string>? FeatureFlags { get; set; } = null;

        [JsonProperty("id", Required = Required.Always)]
        public string Id { get; set; }

        [JsonProperty("name", Required = Required.Always)]
        public string Name { get; set; }

        [JsonProperty("updatedAt", Required = Required.Always)]
        public DateTime UpdatedAt { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class IntegrationOut {\n");
            sb.Append("  CreatedAt: ").Append(CreatedAt).Append('\n');
            sb.Append("  FeatureFlags: ").Append(FeatureFlags).Append('\n');
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("  Name: ").Append(Name).Append('\n');
            sb.Append("  UpdatedAt: ").Append(UpdatedAt).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
