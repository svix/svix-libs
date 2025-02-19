// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
/**
 * Import a list of event types from webhooks defined in an OpenAPI spec.
 *
 * The OpenAPI spec can be specified as either `spec` given the spec as a JSON object, or as `specRaw` (a `string`) which will be parsed as YAML or JSON by the server. Sending neither or both is invalid, resulting in a `400` **Bad Request**.
 */
export interface EventTypeImportOpenApiIn {
  /** If `true`, return the event types that would be modified without actually modifying them. */
  dryRun?: boolean;
  /** If `true`, all existing event types that are not in the spec will be archived. */
  replaceAll?: boolean;
  /** A pre-parsed JSON spec. */
  spec?: any | null;
  /** A string, parsed by the server as YAML or JSON. */
  specRaw?: string | null;
}

export const EventTypeImportOpenApiInSerializer = {
  _fromJsonObject(object: any): EventTypeImportOpenApiIn {
    return {
      dryRun: object["dryRun"],
      replaceAll: object["replaceAll"],
      spec: object["spec"],
      specRaw: object["specRaw"],
    };
  },

  _toJsonObject(self: EventTypeImportOpenApiIn): any {
    return {
      dryRun: self.dryRun,
      replaceAll: self.replaceAll,
      spec: self.spec,
      specRaw: self.specRaw,
    };
  },
};
