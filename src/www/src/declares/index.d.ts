declare interface OrganizationParams {
    name: string;
}

declare interface LanguageTypeData {
    language_type: string;
    code: number;
}

declare interface LanguageData {
    language_type: string;
    code: number;
    color: string;
    percent: number;
}

declare interface RepoWikiParams {
    owner: string;
    object_id: string;
    page_title: string;
}

declare interface RepoWikiHomeRequest {
    owner: string;
    object_id: string;
    page_title: string;
}

// type RepoWikiHomeResponse = {
//     data: RepoWikiPage[],
//     wiki: RepoWikiPage
// }

// type RepoWikiPage = {
//     content: string,
//     date: number,
//     id: string,
//     publisher_id: string,
//     title: string
// }

declare interface RepoUrlParams {
    file: string;
    owner: string;
    object_id: string;
    branch: string;
    tag_name: string;
}

declare interface ResponseRepoFile {
    file: string;
    commit: string;
    author: string;
    date: string;
    message: string;
    fileType: string;
    file_type?: string;
}

declare interface RepositoryRelease {
    id: string;
    commit_id: string;
    publisher_id: string;
    tag_name: string;
    tag_target: string;
    title: string;
    content: string;
    date: number;
    file_id: string;
}

declare interface CommitDiffProps {
    diffData: DiffLineData[];
    header?: boolean;
}

declare interface LineContent {
    type: string;
    content: string;
    leftCount: number | string;
    rightCount: number | string;
    diff_type: string;
    left_line: string;
    right_line: string;
}

declare interface LineData {
    pathName: string;
    title: string;
    data: Array<LineContent>;
    file_name: string;
    file_content: LineContent[];
}

declare interface UserLogoProps {
    user_id: string;
    className?: string;
}

declare interface EditorProps {
    onChange: React.Dispatch<React.SetStateAction<string>>;
    value?: string;
}

declare interface RepositoryTabType {
    name: string;
    key: string;
    src: string;
    checkedSrc: string;
    hide?: boolean;
}

declare interface OrganizationRepositoryProps {
    organization: ResponseOrganizationHome;
}

declare interface PrivilegeBox {
    data: ResponseRepositoryMember;
    onChange: (data: ResponseRepositoryMember, value: string) => void;
}

declare interface FileProps {
    data: DiffResult;
}

declare interface InviteModalProps {
    onConfirm: (value: string) => void;
}

declare interface LanguageColor {
    [key: string]: {
        color: string | null;
        url: string;
    };
}

declare interface CompareInfo {
    commits: CompareCommit[];
    diff: DiffResult[];
    origin: string;
    target: string;
}

declare interface CompareBranch {
    origin: string;
    target: string;
}

declare type UserInfoData = ServiceResponseUserData & {
    id: string;
};

declare interface CommitModel {
    commits: ResponseCommit[];
    owner?: string;
}

declare interface InitModalType {
    title: string;
    show: boolean;
}

declare interface RepoFileList {
    files: ResponseRepoFile[];
    readme?: RepositoryFileReadme;
}

declare interface PaginationParam {
    pageSize: number;
    total: number;
    onChange: Function;
}

declare interface DeleteParam {
    id?: string;
    title: string;
    content: string;
    cb?: Function;
}

declare const __VERSION__: String;

declare type RepoIssueUrlParams = RepoUrlParams & {
    issue_id: string;
};

declare type RepoMergeUrlParams = RepoUrlParams & {
    pull_id: string;
};

declare interface RepoDiffUrlParams {
    file: string;
    owner: string;
    object_id: string;
    hashId: string;
}

declare type classKey = string | { [key: string]: boolean };

declare interface cacheDataStruct {
    data: string;
    ttl: number;
    date: number;
}
declare interface cacheResponse {
    ok: boolean;
    data: string;
}

declare interface PromptDialogRefType {
    setShow: (v: boolean) => void;
    setLoading: (v: boolean) => void;
}
