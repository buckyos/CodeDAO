import {
    DescContentDecoder,
    DescTypeInfo,
    named_id_gen_default,
    named_id_from_base_58,
    named_id_try_from_object_id,
    NamedObject,
    NamedObjectBuilder,
    NamedObjectDecoder,
    NamedObjectDesc,
    NamedObjectId,
    NamedObjectIdDecoder,
    SubDescType,
    NamedObjectDescDecoder,
    BuckyResult,
    Ok,
    DeviceBodyContent,
    DeviceDescContent,
    ObjectId,
    ProtobufBodyContent,
    ProtobufBodyContentDecoder,
    ProtobufDescContent
} from 'cyfs-sdk';
import * as protos from '@src/utils/protos';
import { CustumObjectType } from './rpc_def';

// 1. 定义一个Desc类型信息
export class GitTextObjectDescTypeInfo extends DescTypeInfo {
    obj_type(): number {
        return CustumObjectType.GitText;
    }

    sub_desc_type(): SubDescType {
        return {
            owner_type: 'option', // 是否有主，"disable": 禁用，"option": 可选
            area_type: 'option', // 是否有区域信息，"disable": 禁用，"option": 可选
            author_type: 'option', // 是否有作者，"disable": 禁用，"option": 可选
            // key_type: "single_key"  // 公钥类型，"disable": 禁用，"single_key": 单PublicKey，"mn_key": M-N 公钥对
            // 还不知道怎么用key 暂时先disable
            key_type: 'disable'
        };
    }
}

// 2. 定义一个类型信息常量
const GitTextObject_DESC_TYPE_INFO = new GitTextObjectDescTypeInfo();

// 3. 定义DescContent，继承自DescContent
export class GitTextObjectDescContent extends ProtobufDescContent {
    type_info(): DescTypeInfo {
        return GitTextObject_DESC_TYPE_INFO;
    }

    try_to_proto(): BuckyResult<protos.GitTextDescContent> {
        const target = new protos.GitTextDescContent();
        return Ok(target);
    }
}

// 4. 定义一个DescContent的解码器
export class GitTextObjectDescContentDecoder extends DescContentDecoder<GitTextObjectDescContent> {
    type_info(): DescTypeInfo {
        return GitTextObject_DESC_TYPE_INFO;
    }

    raw_decode(buf: Uint8Array): BuckyResult<[GitTextObjectDescContent, Uint8Array]> {
        const self = new GitTextObjectDescContent();
        const ret: [GitTextObjectDescContent, Uint8Array] = [self, buf];
        return Ok(ret);
    }
}

// 5. 定义一个BodyContent，继承自RawEncode
export class GitTextObjectBodyContent extends ProtobufBodyContent {
    id: string;
    header: string;
    value: string;

    constructor(id: string, header: string, value: string) {
        super();
        this.id = id;
        this.header = header;
        this.value = value;
    }

    try_to_proto(): BuckyResult<protos.GitTextBodyContent> {
        const value = new protos.GitTextBodyContent();
        value.setId(this.id ? this.id : '');
        value.setHeader(this.header ? this.header : '');
        value.setValue(this.value ? this.value : '');
        return Ok(value);
    }
}

// 6. 定义一个BodyContent的解码器
export class GitTextObjectBodyContentDecoder extends ProtobufBodyContentDecoder<
    GitTextObjectBodyContent,
    protos.GitTextBodyContent
> {
    constructor() {
        super(protos.GitTextBodyContent.deserializeBinary);
    }
    try_from_proto(value: protos.GitTextBodyContent): BuckyResult<GitTextObjectBodyContent> {
        const result = new GitTextObjectBodyContent(
            value.getId(),
            value.getHeader(),
            value.getValue()
        );
        return Ok(result);
    }
}

// 7. 定义组合类型
export class GitTextObjectDesc extends NamedObjectDesc<GitTextObjectDescContent> {
    //
}

export class GitTextObjectDescDecoder extends NamedObjectDescDecoder<GitTextObjectDescContent> {
    constructor() {
        super(new GitTextObjectDescContentDecoder());
    }
}

export class GitTextObjectBuilder extends NamedObjectBuilder<
    GitTextObjectDescContent,
    GitTextObjectBodyContent
> {}

// 通过继承的方式具体化
export class GitTextObjectId extends NamedObjectId<
    GitTextObjectDescContent,
    GitTextObjectBodyContent
> {
    static default(): GitTextObjectId {
        return named_id_gen_default(CustumObjectType.GitText);
    }

    static from_base_58(s: string): BuckyResult<GitTextObjectId> {
        return named_id_from_base_58(CustumObjectType.GitText, s);
    }

    static try_from_object_id(id: ObjectId): BuckyResult<GitTextObjectId> {
        return named_id_try_from_object_id(CustumObjectType.GitText, id);
    }
}

export class GitTextObjectIdDecoder extends NamedObjectIdDecoder<
    DeviceDescContent,
    DeviceBodyContent
> {
    constructor() {
        super(CustumObjectType.GitText);
    }
}

// 定义GitTextObject对象
// 提供创建方法和其他自定义方法
export class GitTextObject extends NamedObject<GitTextObjectDescContent, GitTextObjectBodyContent> {
    static create(
        owner: ObjectId,
        dec_id: ObjectId,
        id: string,
        header: string,
        value: string
    ): GitTextObject {
        const desc_content = new GitTextObjectDescContent();
        const body_content = new GitTextObjectBodyContent(id, header, value);
        const builder = new NamedObjectBuilder<GitTextObjectDescContent, GitTextObjectBodyContent>(
            desc_content,
            body_content
        );
        const self = builder.owner(owner).dec_id(dec_id).build(GitTextObject);
        return new GitTextObject(self.desc(), self.body(), self.signs(), self.nonce());
    }

    get id(): string {
        return this.body_expect().content().id;
    }
    get header(): string {
        return this.body_expect().content().header;
    }
    get value(): string {
        return this.body_expect().content().value;
    }
}

// 9. 定义GitTextObject解码器
export class GitTextObjectDecoder extends NamedObjectDecoder<
    GitTextObjectDescContent,
    GitTextObjectBodyContent,
    GitTextObject
> {
    constructor() {
        super(
            new GitTextObjectDescContentDecoder(),
            new GitTextObjectBodyContentDecoder(),
            GitTextObject
        );
    }
}
