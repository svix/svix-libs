// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

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

export const EventExampleInSerializer = {
  _fromJsonObject(object: any): EventExampleIn {
    return {
      eventType: object["eventType"],
      exampleIndex: object["exampleIndex"],
    };
  },

  _toJsonObject(self: EventExampleIn): any {
    return {
      eventType: self.eventType,
      exampleIndex: self.exampleIndex,
    };
  },
};
