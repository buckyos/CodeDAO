import { atom } from 'recoil';

export const commitModelAtom = atom<CommitModel>({
    key: 'commitModel',
    default: {
        commits: []
    }
});