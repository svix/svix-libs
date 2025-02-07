// this file is @generated

export interface EventExampleIn {
  /** The event type's name */
  eventType: string;
  /**
   * If the event type schema contains an array of examples, chooses which one to send.
   *
   * Defaults to the first example. Ignored if the schema doesn't contain an array of examples.
   */
  exampleIndex?: number;
}

export namespace EventExampleInUtil {
  export function _fromJsonObject(object: any): EventExampleIn {
    return {
      eventType: object["eventType"],
      exampleIndex: object["exampleIndex"],
    };
  }

  export function _toJsonObject(self: EventExampleIn): any {
    return {
      eventType: self.eventType,
      exampleIndex: self.exampleIndex,
    };
  }
}
