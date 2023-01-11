// package: 
// file: objects.proto

import * as jspb from "google-protobuf";

export class GitTextDescContent extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GitTextDescContent.AsObject;
  static toObject(includeInstance: boolean, msg: GitTextDescContent): GitTextDescContent.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GitTextDescContent, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GitTextDescContent;
  static deserializeBinaryFromReader(message: GitTextDescContent, reader: jspb.BinaryReader): GitTextDescContent;
}

export namespace GitTextDescContent {
  export type AsObject = {
    id: string,
  }
}

export class GitTextBodyContent extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  getHeader(): string;
  setHeader(value: string): void;

  getValue(): string;
  setValue(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GitTextBodyContent.AsObject;
  static toObject(includeInstance: boolean, msg: GitTextBodyContent): GitTextBodyContent.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GitTextBodyContent, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GitTextBodyContent;
  static deserializeBinaryFromReader(message: GitTextBodyContent, reader: jspb.BinaryReader): GitTextBodyContent;
}

export namespace GitTextBodyContent {
  export type AsObject = {
    id: string,
    header: string,
    value: string,
  }
}

export class PubSubObject extends jspb.Message {
  getAppname(): string;
  setAppname(value: string): void;

  getDecid(): string;
  setDecid(value: string): void;

  getActiontype(): ActionsMap[keyof ActionsMap];
  setActiontype(value: ActionsMap[keyof ActionsMap]): void;

  getActiontarget(): string;
  setActiontarget(value: string): void;

  hasDescribe(): boolean;
  clearDescribe(): void;
  getDescribe(): string;
  setDescribe(value: string): void;

  hasOpenurl(): boolean;
  clearOpenurl(): void;
  getOpenurl(): string;
  setOpenurl(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PubSubObject.AsObject;
  static toObject(includeInstance: boolean, msg: PubSubObject): PubSubObject.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: PubSubObject, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PubSubObject;
  static deserializeBinaryFromReader(message: PubSubObject, reader: jspb.BinaryReader): PubSubObject;
}

export namespace PubSubObject {
  export type AsObject = {
    appname: string,
    decid: string,
    actiontype: ActionsMap[keyof ActionsMap],
    actiontarget: string,
    describe: string,
    openurl: string,
  }
}

export class NoneObject extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): NoneObject.AsObject;
  static toObject(includeInstance: boolean, msg: NoneObject): NoneObject.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: NoneObject, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): NoneObject;
  static deserializeBinaryFromReader(message: NoneObject, reader: jspb.BinaryReader): NoneObject;
}

export namespace NoneObject {
  export type AsObject = {
  }
}

export interface ActionsMap {
  CREATE: 0;
  UPDATE: 1;
  RETRIEVE: 2;
  DELETE: 3;
}

export const Actions: ActionsMap;

