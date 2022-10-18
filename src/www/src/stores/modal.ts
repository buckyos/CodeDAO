import { observable } from 'mobx';
import { atom } from 'recoil';

export const modalStore = observable<{
    title: string,
    show: boolean,
    ref: React.MutableRefObject<null>|null,
    close:() => void,
        }>({
            title: '',
            show: false,
            ref: null,
            close() {
                this.show = false;
            }
        });

export const inviteModalStoreAtom = atom<InitModalType>({
    key: 'inviteModalStore',
    default: {
        title: '',
        show: false
    }
});