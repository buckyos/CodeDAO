import { cacheGet, cacheSet } from '../utils/cache';
// import {cacheResponse} from "../../@types";
import { atom } from 'recoil';

export class UserInfo {
	private cacheKey = 'user_info';

	checkCache(ownerId: string): boolean {
	    const result: cacheResponse = cacheGet(this.cacheKey);
	    if (result.ok) {
	        const value: ServiceResponseUserData = JSON.parse(result.data);
	        this.setData(value, ownerId);
	        return result.ok;
	    }

	    return false;
	}

	setData(data: ServiceResponseUserData, ownerId: string) {
	    cacheSet(this.cacheKey, data, 3600);
	}
}

export const userInfoAtom = atom<UserInfoData>({
    key: 'userInfo',
    default: {
        name: '',
        id: '',
        email: '',
        date: 0,
        userId: '',
        owner: ''
    }
});

export const UserInfoStore = new UserInfo();
