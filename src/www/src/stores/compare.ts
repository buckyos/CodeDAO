import { atom, useSetRecoilState } from 'recoil';

export const repositoryCompareInfoAtom = atom<CompareInfo>({
    key: 'repositoryCompareInfo',
    default: {
        commits: [],
        diff: [],
        origin: '',
        target: ''
    }
});

export function useCleanCompareInfo() {
    const setCompareInfo = useSetRecoilState(repositoryCompareInfoAtom);

    return function () {
        setCompareInfo({
            commits: [],
            diff: [],
            origin: '',
            target: ''
        }); 
    };
}