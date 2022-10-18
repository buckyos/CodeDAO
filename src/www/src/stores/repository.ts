
import {
    atom,
    useSetRecoilState
} from 'recoil';

export const repositorysAtom = atom<ResponseRepository[]>({
    key: 'repositorys',
    default: []
});

export const repositoryAtom = atom<ResponseRepository>({
    key: 'repository',
    default: {
        owner: '',
        id: '',
        name: '',
        description: '',
        is_private: 1,
        init: 0,
        binary_id: '',
        fork_from: '',
        author_type: '',
        author_name: ''
        // is_setting: false,
        // star_count: 0,
    }
});

export const branchesAtom = atom<string[]>({
    key: 'branches',
    default: []
});

export const repositoryLastCommitAtom = atom<ResponseRepositoryCommit>({
    key: 'lastCommit',
    default: {
        author: '',
        message: '',
        commit: '',
        date: 0
    }
});

export const repositoryCurrentBranchAtom = atom<string>({
    key: 'repositoryCurrentBranch',
    default: 'master'
});

export const repositoryReleaseCountAtom = atom<number>({
    key: 'repositoryReleaseCount',
    default: 0
});

export const repositoryCommitCountAtom = atom<number>({
    key: 'repositoryCommitCount',
    default: 0
});

export const repositoryStarCountAtom = atom<number>({
    key: 'repositoryStarCount',
    default: 0
});

export const repositorySettingAtom = atom<boolean>({
    key: 'repositorySetting',
    default: false
});

export const repositoryChildPath = atom<string>({
    key: 'repositoryChildPath',
    default: ''
});

export const repositorySourceFiles = atom<ResponseRepoFile[]>({
    key: 'repositorySourceFiles',
    default: []
});

export const repositoryReadme = atom<RepositoryFileReadme>({
    key: 'repositoryReadme',
    default: { type: '', content: '' }
});

export function useCleanSwitchPath() {
    const setRepositoryReadme = useSetRecoilState(repositoryReadme);

    return function () {
        setRepositoryReadme({ type: '', content: '' });
    };
}

export function useCleanRepoHome() {
    const setRepoHomeData = useSetRecoilState(repositoryAtom);
    const setLastCommit = useSetRecoilState(repositoryLastCommitAtom);
    const setReleaseCount = useSetRecoilState(repositoryReleaseCountAtom);
    const setCommitCount = useSetRecoilState(repositoryCommitCountAtom);
    const setStarCount = useSetRecoilState(repositoryStarCountAtom);
    const setRepositorySetting = useSetRecoilState(repositorySettingAtom);

    return function () {
        setRepoHomeData({
            owner: '',
            id: '',
            name: '',
            description: '',
            is_private: 1,
            init: 0,
            binary_id: '',
            fork_from: '',
            author_type: '',
            author_name: ''
        });
        setLastCommit({
            author: '',
            message: '',
            commit: '',
            date: 0
        });
        setReleaseCount(0);
        setCommitCount(0);
        setStarCount(0);
        setRepositorySetting(false);

    };
}

