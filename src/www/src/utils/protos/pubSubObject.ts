// ObjectName: PubSubObject
import * as cyfs from 'cyfs-sdk';
import { AppObjectType } from '../../types';
import * as protos from './objects_pb';
import { DEC_ID } from '../../constants';

// type的前16位是系统保留类型，应用的对象type应该大于32767
export const PUBSUB_OBJECT_TYPE = AppObjectType.PUBSUB_OBJECT;

export class PubSubObjectDescTypeInfo extends cyfs.DescTypeInfo {
    public obj_type(): number {
        return PUBSUB_OBJECT_TYPE;
    }

    public sub_desc_type(): cyfs.SubDescType {
        // default
        return {
            owner_type: 'option',
            area_type: 'option',
            author_type: 'option',
            key_type: 'disable'
        };
    }
}

const PUBSUB_OBJECT_DESC_TYPE_INFO = new PubSubObjectDescTypeInfo();

export class PubSubObjectDescContent extends cyfs.ProtobufDescContent {
    private m_appName: string;
    private m_decId: string;
    private m_actionType: protos.ActionsMap[keyof protos.ActionsMap];
    private m_actionTarget: string;
    private m_describe?: string;
    private m_openURL?: string;
    public constructor(param: {
        appName: string;
        decId: string;
        actionType: protos.ActionsMap[keyof protos.ActionsMap];
        actionTarget: string;
        openURL?: string;
        describe?: string;
    }) {
        super();
        this.m_appName = param.appName;
        this.m_decId = param.decId;
        this.m_actionType = param.actionType;
        this.m_openURL = param.openURL;
        this.m_describe = param.describe;
        this.m_actionTarget = param.actionTarget;
    }

    public type_info(): cyfs.DescTypeInfo {
        return PUBSUB_OBJECT_DESC_TYPE_INFO;
    }

    public try_to_proto(): cyfs.BuckyResult<protos.PubSubObject> {
        const target = new protos.PubSubObject();
        target.setAppname(this.m_appName);
        target.setDecid(this.m_decId);
        target.setActiontype(this.m_actionType);
        target.setActiontarget(this.m_actionTarget);
        if (this.m_describe) target.setDescribe(this.m_describe);
        if (this.m_openURL) target.setOpenurl(this.m_openURL);
        return cyfs.Ok(target);
    }

    public get appName(): string {
        return this.m_appName;
    }

    public get decId(): string {
        return this.m_decId;
    }

    public get actionType(): protos.ActionsMap[keyof protos.ActionsMap] {
        return this.m_actionType;
    }

    public get openURL() {
        return this.m_openURL ? this.m_openURL : '';
    }

    public get describe() {
        return this.m_describe ? this.m_describe : '';
    }
    public get actionTarget() {
        return this.m_actionTarget;
    }
}

export class PubSubObjectDescContentDecoder extends cyfs.ProtobufDescContentDecoder<
    PubSubObjectDescContent,
    protos.PubSubObject
> {
    public constructor() {
        super(protos.PubSubObject.deserializeBinary);
    }

    public type_info(): cyfs.DescTypeInfo {
        return PUBSUB_OBJECT_DESC_TYPE_INFO;
    }

    public try_from_proto(
        psObject: protos.PubSubObject
    ): cyfs.BuckyResult<PubSubObjectDescContent> {
        const appName = psObject.getAppname();
        const decId = psObject.getDecid();
        const actionType = psObject.getActiontype();
        const openURL = psObject.getOpenurl();
        const describe = psObject.getDescribe();
        const actionTarget = psObject.getActiontarget();
        return cyfs.Ok(
            new PubSubObjectDescContent({
                appName,
                decId,
                actionType,
                openURL,
                describe,
                actionTarget
            })
        );
    }
}

export class PubSubObjectDesc extends cyfs.NamedObjectDesc<PubSubObjectDescContent> {
    // default
}

export class PubSubObjectDescDecoder extends cyfs.NamedObjectDescDecoder<PubSubObjectDescContent> {
    // default
}

export class PubSubObjectBodyContent extends cyfs.ProtobufBodyContent {
    public constructor() {
        super();
    }

    public try_to_proto(): cyfs.BuckyResult<protos.NoneObject> {
        return cyfs.Ok(new protos.NoneObject());
    }
}

export class PubSubObjectBodyContentDecoder extends cyfs.ProtobufBodyContentDecoder<
    PubSubObjectBodyContent,
    protos.NoneObject
> {
    public constructor() {
        super(protos.NoneObject.deserializeBinary);
    }

    public try_from_proto(value: protos.NoneObject): cyfs.BuckyResult<PubSubObjectBodyContent> {
        return cyfs.Ok(new PubSubObjectBodyContent());
    }
}

export class PubSubObjectBuilder extends cyfs.NamedObjectBuilder<
    PubSubObjectDescContent,
    PubSubObjectBodyContent
> {
    // default
}

export class PubSubObjectId extends cyfs.NamedObjectId<
    PubSubObjectDescContent,
    PubSubObjectBodyContent
> {
    public constructor(id: cyfs.ObjectId) {
        super(PUBSUB_OBJECT_TYPE, id);
    }
    // default
}

export class PubSubObjectIdDecoder extends cyfs.NamedObjectIdDecoder<
    PubSubObjectDescContent,
    PubSubObjectBodyContent
> {
    public constructor() {
        super(PUBSUB_OBJECT_TYPE);
    }
    // default
}

export class PubSubObject extends cyfs.NamedObject<
    PubSubObjectDescContent,
    PubSubObjectBodyContent
> {
    public static create(param: {
        appName: string;
        decId: string;
        actionType: protos.ActionsMap[keyof protos.ActionsMap];
        actionTarget: string;
        owner: cyfs.ObjectId;
        openURL?: string;
        describe?: string;
    }): PubSubObject {
        const descContent = new PubSubObjectDescContent(param);
        const bodyContent = new PubSubObjectBodyContent();
        const builder = new PubSubObjectBuilder(descContent, bodyContent);
        return builder.dec_id(DEC_ID).owner(param.owner).build(PubSubObject);
    }

    public get appName(): string {
        return this.desc().content().appName;
    }

    public get decId(): string {
        return this.desc().content().decId;
    }

    public get actionType(): protos.ActionsMap[keyof protos.ActionsMap] {
        return this.desc().content().actionType;
    }

    public get actionTarget(): string {
        return this.desc().content().actionTarget;
    }

    public get openURL(): string {
        return this.desc().content().openURL;
    }

    public get describe(): string {
        return this.desc().content().describe;
    }
}

export class PubSubObjectDecoder extends cyfs.NamedObjectDecoder<
    PubSubObjectDescContent,
    PubSubObjectBodyContent,
    PubSubObject
> {
    public constructor() {
        super(
            new PubSubObjectDescContentDecoder(),
            new PubSubObjectBodyContentDecoder(),
            PubSubObject
        );
    }
}
