// this file is @generated
using System.Runtime.Serialization;
using System.Text;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Svix.Models
{
    [JsonConverter(typeof(IngestSourceOutConverter))]
    public class IngestSourceOut
    {
        [JsonProperty("createdAt", Required = Required.Always)]
        public DateTime CreatedAt { get; set; }

        [JsonProperty("id", Required = Required.Always)]
        public string Id { get; set; }

        [JsonProperty("ingestUrl")]
        public string? IngestUrl { get; set; } = null;

        [JsonProperty("name", Required = Required.Always)]
        public string Name { get; set; }

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("updatedAt", Required = Required.Always)]
        public DateTime UpdatedAt { get; set; }

        [JsonIgnore]
        public IngestSourceOutConfig Config { get; set; }

        [JsonProperty("type")]
        private string Type => Config.GetDiscriminator();

        [JsonProperty("config")]
        private object realConfig
        {
            get => Config.GetContent();
            set => Config.SetContent(value);
        }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();

            sb.Append("class IngestSourceOut {\n");
            sb.Append("  CreatedAt: ").Append(CreatedAt).Append('\n');
            sb.Append("  Id: ").Append(Id).Append('\n');
            sb.Append("  IngestUrl: ").Append(IngestUrl).Append('\n');
            sb.Append("  Name: ").Append(Name).Append('\n');
            sb.Append("  Uid: ").Append(Uid).Append('\n');
            sb.Append("  UpdatedAt: ").Append(UpdatedAt).Append('\n');
            sb.Append("  Config: ").Append(Config).Append('\n');
            sb.Append("}\n");
            return sb.ToString();
        }
    }

    public class IngestSourceOutConfig
    {
        private object _value;
        private readonly ConfigType _type;

        internal string GetDiscriminator()
        {
            var memberInfo = typeof(ConfigType).GetMember(_type.ToString());
            var enumMember = memberInfo[0]
                .GetCustomAttributes(false)
                .OfType<EnumMemberAttribute>()
                .FirstOrDefault();
            return enumMember?.Value ?? _type.ToString().ToLower();
        }

        internal void SetContent(object value)
        {
            _value = value;
        }

        public object GetContent()
        {
            return _value;
        }

        private IngestSourceOutConfig(object value, ConfigType type)
        {
            _value = value;
            _type = type;
        }

        public static IngestSourceOutConfig GenericWebhook() =>
            new(new Dictionary<string, string>(), ConfigType.GenericWebhook);

        public static IngestSourceOutConfig Cron(CronConfig cronConfig) =>
            new(cronConfig, ConfigType.Cron);

        public static IngestSourceOutConfig AdobeSign(AdobeSignConfigOut adobeSignConfigOut) =>
            new(adobeSignConfigOut, ConfigType.AdobeSign);

        public static IngestSourceOutConfig Beehiiv(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Beehiiv);

        public static IngestSourceOutConfig Brex(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Brex);

        public static IngestSourceOutConfig Clerk(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Clerk);

        public static IngestSourceOutConfig Docusign(DocusignConfigOut docusignConfigOut) =>
            new(docusignConfigOut, ConfigType.Docusign);

        public static IngestSourceOutConfig Github(GithubConfigOut githubConfigOut) =>
            new(githubConfigOut, ConfigType.Github);

        public static IngestSourceOutConfig Guesty(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Guesty);

        public static IngestSourceOutConfig Hubspot(HubspotConfigOut hubspotConfigOut) =>
            new(hubspotConfigOut, ConfigType.Hubspot);

        public static IngestSourceOutConfig IncidentIo(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.IncidentIo);

        public static IngestSourceOutConfig Lithic(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Lithic);

        public static IngestSourceOutConfig Nash(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Nash);

        public static IngestSourceOutConfig PandaDoc(PandaDocConfigOut pandaDocConfigOut) =>
            new(pandaDocConfigOut, ConfigType.PandaDoc);

        public static IngestSourceOutConfig Pleo(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Pleo);

        public static IngestSourceOutConfig Replicate(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Replicate);

        public static IngestSourceOutConfig Resend(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Resend);

        public static IngestSourceOutConfig Safebase(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Safebase);

        public static IngestSourceOutConfig Sardine(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Sardine);

        public static IngestSourceOutConfig Segment(SegmentConfigOut segmentConfigOut) =>
            new(segmentConfigOut, ConfigType.Segment);

        public static IngestSourceOutConfig Shopify(ShopifyConfigOut shopifyConfigOut) =>
            new(shopifyConfigOut, ConfigType.Shopify);

        public static IngestSourceOutConfig Slack(SlackConfigOut slackConfigOut) =>
            new(slackConfigOut, ConfigType.Slack);

        public static IngestSourceOutConfig Stripe(StripeConfigOut stripeConfigOut) =>
            new(stripeConfigOut, ConfigType.Stripe);

        public static IngestSourceOutConfig Stych(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Stych);

        public static IngestSourceOutConfig Svix(SvixConfigOut svixConfigOut) =>
            new(svixConfigOut, ConfigType.Svix);

        public static IngestSourceOutConfig Zoom(ZoomConfigOut zoomConfigOut) =>
            new(zoomConfigOut, ConfigType.Zoom);

        private enum ConfigType
        {
            [EnumMember(Value = "generic-webhook")]
            GenericWebhook,

            [EnumMember(Value = "cron")]
            Cron,

            [EnumMember(Value = "adobe-sign")]
            AdobeSign,

            [EnumMember(Value = "beehiiv")]
            Beehiiv,

            [EnumMember(Value = "brex")]
            Brex,

            [EnumMember(Value = "clerk")]
            Clerk,

            [EnumMember(Value = "docusign")]
            Docusign,

            [EnumMember(Value = "github")]
            Github,

            [EnumMember(Value = "guesty")]
            Guesty,

            [EnumMember(Value = "hubspot")]
            Hubspot,

            [EnumMember(Value = "incident-io")]
            IncidentIo,

            [EnumMember(Value = "lithic")]
            Lithic,

            [EnumMember(Value = "nash")]
            Nash,

            [EnumMember(Value = "panda-doc")]
            PandaDoc,

            [EnumMember(Value = "pleo")]
            Pleo,

            [EnumMember(Value = "replicate")]
            Replicate,

            [EnumMember(Value = "resend")]
            Resend,

            [EnumMember(Value = "safebase")]
            Safebase,

            [EnumMember(Value = "sardine")]
            Sardine,

            [EnumMember(Value = "segment")]
            Segment,

            [EnumMember(Value = "shopify")]
            Shopify,

            [EnumMember(Value = "slack")]
            Slack,

            [EnumMember(Value = "stripe")]
            Stripe,

            [EnumMember(Value = "stych")]
            Stych,

            [EnumMember(Value = "svix")]
            Svix,

            [EnumMember(Value = "zoom")]
            Zoom,
        }

        public TResult Match<TResult>(
            Func<TResult> onGenericWebhook,
            Func<CronConfig, TResult> onCron,
            Func<AdobeSignConfigOut, TResult> onAdobeSign,
            Func<SvixConfigOut, TResult> onBeehiiv,
            Func<SvixConfigOut, TResult> onBrex,
            Func<SvixConfigOut, TResult> onClerk,
            Func<DocusignConfigOut, TResult> onDocusign,
            Func<GithubConfigOut, TResult> onGithub,
            Func<SvixConfigOut, TResult> onGuesty,
            Func<HubspotConfigOut, TResult> onHubspot,
            Func<SvixConfigOut, TResult> onIncidentIo,
            Func<SvixConfigOut, TResult> onLithic,
            Func<SvixConfigOut, TResult> onNash,
            Func<PandaDocConfigOut, TResult> onPandaDoc,
            Func<SvixConfigOut, TResult> onPleo,
            Func<SvixConfigOut, TResult> onReplicate,
            Func<SvixConfigOut, TResult> onResend,
            Func<SvixConfigOut, TResult> onSafebase,
            Func<SvixConfigOut, TResult> onSardine,
            Func<SegmentConfigOut, TResult> onSegment,
            Func<ShopifyConfigOut, TResult> onShopify,
            Func<SlackConfigOut, TResult> onSlack,
            Func<StripeConfigOut, TResult> onStripe,
            Func<SvixConfigOut, TResult> onStych,
            Func<SvixConfigOut, TResult> onSvix,
            Func<ZoomConfigOut, TResult> onZoom
        )
        {
            return _type switch
            {
                ConfigType.GenericWebhook => onGenericWebhook(),
                ConfigType.Cron => onCron((CronConfig)_value),
                ConfigType.AdobeSign => onAdobeSign((AdobeSignConfigOut)_value),
                ConfigType.Beehiiv => onBeehiiv((SvixConfigOut)_value),
                ConfigType.Brex => onBrex((SvixConfigOut)_value),
                ConfigType.Clerk => onClerk((SvixConfigOut)_value),
                ConfigType.Docusign => onDocusign((DocusignConfigOut)_value),
                ConfigType.Github => onGithub((GithubConfigOut)_value),
                ConfigType.Guesty => onGuesty((SvixConfigOut)_value),
                ConfigType.Hubspot => onHubspot((HubspotConfigOut)_value),
                ConfigType.IncidentIo => onIncidentIo((SvixConfigOut)_value),
                ConfigType.Lithic => onLithic((SvixConfigOut)_value),
                ConfigType.Nash => onNash((SvixConfigOut)_value),
                ConfigType.PandaDoc => onPandaDoc((PandaDocConfigOut)_value),
                ConfigType.Pleo => onPleo((SvixConfigOut)_value),
                ConfigType.Replicate => onReplicate((SvixConfigOut)_value),
                ConfigType.Resend => onResend((SvixConfigOut)_value),
                ConfigType.Safebase => onSafebase((SvixConfigOut)_value),
                ConfigType.Sardine => onSardine((SvixConfigOut)_value),
                ConfigType.Segment => onSegment((SegmentConfigOut)_value),
                ConfigType.Shopify => onShopify((ShopifyConfigOut)_value),
                ConfigType.Slack => onSlack((SlackConfigOut)_value),
                ConfigType.Stripe => onStripe((StripeConfigOut)_value),
                ConfigType.Stych => onStych((SvixConfigOut)_value),
                ConfigType.Svix => onSvix((SvixConfigOut)_value),
                ConfigType.Zoom => onZoom((ZoomConfigOut)_value),
                // unreachable
                _ => throw new InvalidOperationException("Unknown config type"),
            };
        }

        public void Switch(
            Action onGenericWebhook,
            Action<CronConfig> onCron,
            Action<AdobeSignConfigOut> onAdobeSign,
            Action<SvixConfigOut> onBeehiiv,
            Action<SvixConfigOut> onBrex,
            Action<SvixConfigOut> onClerk,
            Action<DocusignConfigOut> onDocusign,
            Action<GithubConfigOut> onGithub,
            Action<SvixConfigOut> onGuesty,
            Action<HubspotConfigOut> onHubspot,
            Action<SvixConfigOut> onIncidentIo,
            Action<SvixConfigOut> onLithic,
            Action<SvixConfigOut> onNash,
            Action<PandaDocConfigOut> onPandaDoc,
            Action<SvixConfigOut> onPleo,
            Action<SvixConfigOut> onReplicate,
            Action<SvixConfigOut> onResend,
            Action<SvixConfigOut> onSafebase,
            Action<SvixConfigOut> onSardine,
            Action<SegmentConfigOut> onSegment,
            Action<ShopifyConfigOut> onShopify,
            Action<SlackConfigOut> onSlack,
            Action<StripeConfigOut> onStripe,
            Action<SvixConfigOut> onStych,
            Action<SvixConfigOut> onSvix,
            Action<ZoomConfigOut> onZoom
        )
        {
            switch (_type)
            {
                case ConfigType.GenericWebhook:
                    onGenericWebhook();
                    break;
                case ConfigType.Cron:
                    onCron((CronConfig)_value);
                    break;
                case ConfigType.AdobeSign:
                    onAdobeSign((AdobeSignConfigOut)_value);
                    break;
                case ConfigType.Beehiiv:
                    onBeehiiv((SvixConfigOut)_value);
                    break;
                case ConfigType.Brex:
                    onBrex((SvixConfigOut)_value);
                    break;
                case ConfigType.Clerk:
                    onClerk((SvixConfigOut)_value);
                    break;
                case ConfigType.Docusign:
                    onDocusign((DocusignConfigOut)_value);
                    break;
                case ConfigType.Github:
                    onGithub((GithubConfigOut)_value);
                    break;
                case ConfigType.Guesty:
                    onGuesty((SvixConfigOut)_value);
                    break;
                case ConfigType.Hubspot:
                    onHubspot((HubspotConfigOut)_value);
                    break;
                case ConfigType.IncidentIo:
                    onIncidentIo((SvixConfigOut)_value);
                    break;
                case ConfigType.Lithic:
                    onLithic((SvixConfigOut)_value);
                    break;
                case ConfigType.Nash:
                    onNash((SvixConfigOut)_value);
                    break;
                case ConfigType.PandaDoc:
                    onPandaDoc((PandaDocConfigOut)_value);
                    break;
                case ConfigType.Pleo:
                    onPleo((SvixConfigOut)_value);
                    break;
                case ConfigType.Replicate:
                    onReplicate((SvixConfigOut)_value);
                    break;
                case ConfigType.Resend:
                    onResend((SvixConfigOut)_value);
                    break;
                case ConfigType.Safebase:
                    onSafebase((SvixConfigOut)_value);
                    break;
                case ConfigType.Sardine:
                    onSardine((SvixConfigOut)_value);
                    break;
                case ConfigType.Segment:
                    onSegment((SegmentConfigOut)_value);
                    break;
                case ConfigType.Shopify:
                    onShopify((ShopifyConfigOut)_value);
                    break;
                case ConfigType.Slack:
                    onSlack((SlackConfigOut)_value);
                    break;
                case ConfigType.Stripe:
                    onStripe((StripeConfigOut)_value);
                    break;
                case ConfigType.Stych:
                    onStych((SvixConfigOut)_value);
                    break;
                case ConfigType.Svix:
                    onSvix((SvixConfigOut)_value);
                    break;
                case ConfigType.Zoom:
                    onZoom((ZoomConfigOut)_value);
                    break;
                default:
                    // unreachable
                    throw new InvalidOperationException("Unknown config type");
            }
        }
    }

    internal class IngestSourceOutSurrogate
    {
        [JsonProperty("createdAt", Required = Required.Always)]
        public DateTime CreatedAt { get; set; }

        [JsonProperty("id", Required = Required.Always)]
        public string Id { get; set; }

        [JsonProperty("ingestUrl")]
        public string? IngestUrl { get; set; } = null;

        [JsonProperty("name", Required = Required.Always)]
        public string Name { get; set; }

        [JsonProperty("uid")]
        public string? Uid { get; set; } = null;

        [JsonProperty("updatedAt", Required = Required.Always)]
        public DateTime UpdatedAt { get; set; }

        [JsonProperty("type", Required = Required.Always)]
        public string Type { get; set; }

        [JsonProperty("config", Required = Required.Always)]
        public JObject Config { get; set; }
    }

    public class IngestSourceOutConverter : JsonConverter
    {
        public override bool CanConvert(Type objectType)
        {
            return objectType == typeof(IngestSourceOut);
        }

        public override bool CanWrite
        {
            get { return false; }
        }

        public override void WriteJson(JsonWriter writer, object? value, JsonSerializer serializer)
        {
            // unreachable: CanWrite tells Newtonsoft not to call this method
            throw new NotImplementedException();
        }

        public override object ReadJson(
            JsonReader reader,
            Type objectType,
            object? existingValue,
            JsonSerializer serializer
        )
        {
            var surrogate =
                serializer.Deserialize<IngestSourceOutSurrogate>(reader)
                ?? throw new JsonSerializationException(
                    "Failed to deserialize JSON to IngestSourceOutSurrogate"
                );
            if (
                !typeMap.TryGetValue(
                    surrogate.Type,
                    out Func<(JObject, string), IngestSourceOutConfig>? func
                )
            )
            {
                throw new JsonSerializationException(
                    $"Unexpected type {surrogate.Type} for IngestSourceOutConfig.config"
                );
            }

            IngestSourceOutConfig config = func((surrogate.Config, surrogate.Type));

            return new IngestSourceOut
            {
                CreatedAt = surrogate.CreatedAt,
                Id = surrogate.Id,
                IngestUrl = surrogate.IngestUrl,
                Name = surrogate.Name,
                Uid = surrogate.Uid,
                UpdatedAt = surrogate.UpdatedAt,
                Config = config,
            };
        }

        private static T ToObj<T>((JObject, string) args)
        {
            var loadedObj =
                args.Item1.ToObject<T>()
                ?? throw new JsonSerializationException(
                    $"Failed to deserialize {args.Item2} config"
                );
            return loadedObj;
        }

        private readonly Dictionary<
            string,
            Func<(JObject, string), IngestSourceOutConfig>
        > typeMap = new()
        {
            ["generic-webhook"] = c => IngestSourceOutConfig.GenericWebhook(),
            ["cron"] = c => IngestSourceOutConfig.Cron(ToObj<CronConfig>(c)),
            ["adobe-sign"] = c => IngestSourceOutConfig.AdobeSign(ToObj<AdobeSignConfigOut>(c)),
            ["beehiiv"] = c => IngestSourceOutConfig.Beehiiv(ToObj<SvixConfigOut>(c)),
            ["brex"] = c => IngestSourceOutConfig.Brex(ToObj<SvixConfigOut>(c)),
            ["clerk"] = c => IngestSourceOutConfig.Clerk(ToObj<SvixConfigOut>(c)),
            ["docusign"] = c => IngestSourceOutConfig.Docusign(ToObj<DocusignConfigOut>(c)),
            ["github"] = c => IngestSourceOutConfig.Github(ToObj<GithubConfigOut>(c)),
            ["guesty"] = c => IngestSourceOutConfig.Guesty(ToObj<SvixConfigOut>(c)),
            ["hubspot"] = c => IngestSourceOutConfig.Hubspot(ToObj<HubspotConfigOut>(c)),
            ["incident-io"] = c => IngestSourceOutConfig.IncidentIo(ToObj<SvixConfigOut>(c)),
            ["lithic"] = c => IngestSourceOutConfig.Lithic(ToObj<SvixConfigOut>(c)),
            ["nash"] = c => IngestSourceOutConfig.Nash(ToObj<SvixConfigOut>(c)),
            ["panda-doc"] = c => IngestSourceOutConfig.PandaDoc(ToObj<PandaDocConfigOut>(c)),
            ["pleo"] = c => IngestSourceOutConfig.Pleo(ToObj<SvixConfigOut>(c)),
            ["replicate"] = c => IngestSourceOutConfig.Replicate(ToObj<SvixConfigOut>(c)),
            ["resend"] = c => IngestSourceOutConfig.Resend(ToObj<SvixConfigOut>(c)),
            ["safebase"] = c => IngestSourceOutConfig.Safebase(ToObj<SvixConfigOut>(c)),
            ["sardine"] = c => IngestSourceOutConfig.Sardine(ToObj<SvixConfigOut>(c)),
            ["segment"] = c => IngestSourceOutConfig.Segment(ToObj<SegmentConfigOut>(c)),
            ["shopify"] = c => IngestSourceOutConfig.Shopify(ToObj<ShopifyConfigOut>(c)),
            ["slack"] = c => IngestSourceOutConfig.Slack(ToObj<SlackConfigOut>(c)),
            ["stripe"] = c => IngestSourceOutConfig.Stripe(ToObj<StripeConfigOut>(c)),
            ["stych"] = c => IngestSourceOutConfig.Stych(ToObj<SvixConfigOut>(c)),
            ["svix"] = c => IngestSourceOutConfig.Svix(ToObj<SvixConfigOut>(c)),
            ["zoom"] = c => IngestSourceOutConfig.Zoom(ToObj<ZoomConfigOut>(c)),
        };
    }
}
