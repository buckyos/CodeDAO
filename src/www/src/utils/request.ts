import {
    NONAPILevel,
    NONObjectInfo,
    NONPostObjectOutputRequest,
    NONPostObjectOutputResponse,
    ObjectId,
    RequestGlobalStatePath,
} from 'cyfs-sdk';
import { stack, stackInfo } from './stack';
import { GitTextObject, GitTextObjectDecoder } from '@src/types/text';


export async function requestLocal<S>(route: string, data: Object) {
    return generateRequest<S>(stackInfo.ood_device_id.object_id)(route, data);
}

export async function requestTarget<S>(route: string, data: Object, owner: string, id: string) {
    const target: ObjectId = await stackInfo.getTargetDeviceFromString(id, owner);
    console.error('requestTarget target', target.to_base_58());
    return generateRequest<S>(target)(route, data);
}

// // requestTargetUser
// // 已知 对端的owner id
// export async function requestTargetUser<S>(route: string, data: Object, user_id: string) {
//     const target: ObjectId = ObjectId.from_base_58(user_id).unwrap()
//     return generateRequest<S>(target)(route, data)
// }

export function generateRequest<S>(
    target: ObjectId
): (route: string, data: Object) => Promise<RequestTargetCommonResponse<S>> {
    return async function (route: string, data: Object): Promise<RequestTargetCommonResponse<S>> {
        const dataString: string = JSON.stringify(data);
        console.log(`route: [${route}], request data: ${dataString}`);

        const obj = GitTextObject.create(
            stackInfo.owner,
            stackInfo.appID,
            route,
            'header',
            dataString
        );
        const object_id = obj.desc().calculate_id();
        const object_raw = obj.to_vec().unwrap();
        const req_path = new RequestGlobalStatePath(stackInfo.appID, "cyfs-git-app-handler");

        const req: NONPostObjectOutputRequest = {
            common: {
                dec_id: stackInfo.appID,
                req_path: req_path,
                flags: 0,
                level: NONAPILevel.Router,
                target: target
            },
            object: new NONObjectInfo(object_id, object_raw)
        };

        const resp = await stack.non_service().post_object(req);
        if (resp.err) {
            console.error('requestTarget', resp);
            return { err: true, msg: 'post data error', status: -1 };
        }

        const r: NONPostObjectOutputResponse = resp.unwrap();
        const decodeResp = new GitTextObjectDecoder().raw_decode(r.object!.object_raw);
        const [text]: [GitTextObject, Uint8Array] = decodeResp.unwrap();

        console.log(`[${route}] get response data: ${text.value}`);
        const responseData: RequestTargetCommonResponse<S> = JSON.parse(text.value);
        return responseData;
    };
}
