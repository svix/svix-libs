// this file is @generated
using System.Text;
using Newtonsoft.Json;

namespace Svix.Models
{
    public class IntegrationIn
    {
        [JsonProperty("featureFlags")]
        public List<string>? FeatureFlags { get; set; } = null;

        [JsonProperty("name", Required = Required.Always)]
        public string Name { get; set; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class IntegrationIn {\n");
            sb.Append("  FeatureFlags: ").Append(FeatureFlags).Append('\n');
            sb.Append("  Name: ").Append(Name).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }
}
