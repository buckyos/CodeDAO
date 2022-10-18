export const CustumObjectType = {
    Repository: 34498,
    Commit: 32801,
    Issue: 32902,
    Organization: 32904,
    OrganizationMember: 32905,
    GlobalRepository: 32805,
    User: 32806,
    Branch: 32807,
    Fork: 32808,
    GitSource: 32811,
    MergeRequest: 32812,
    RepositoryMember: 65527,
    RepositorySetting: 65528,
    RepositoryRelease: 65529,
    RepositoryWiki: 65530,
    Star: 65534,
    ErrObjType: 65535,

    GitText: 32810
};

export enum TreeFileType {
    File = 0,
    Dir = 1
}

export enum RepositoryType {
    Public = 0,
    Private = 1
}

export enum AuthorType {
    User = 'user',
    Org = 'org'
}

export enum SetStateType {
    Private = 'setprivate',
    Public = 'setpublic'
}

declare global {
    interface IRef {
        branch: string;
        hash: string;
    }

    interface RemoteRepository {
        owner: string;
        author_name: string;
        author_type: string;
        description: string;
        id: string;
        init: number;
        is_private: number;
        name: string;
    }

    interface ResponseGitLsTree {
        type: TreeFileType;
        treeData?: ResponseGitLsTreeItem[];
        content: string;
    }

    interface ResponseGitLsTreeItem {
        fileMode: string;
        gitObjectType: string;
        fileHash: string;
        fileName: string;
    }

    interface ResponseOrganizationList {
        id: string;
        name: string;
        email: string;
        date: number;
        ownerId: string;
    }
    type ResponseOrganizationHome = ResponseOrganizationList & {
        member_count: number;
        is_show_add: boolean;
        repository_count: number;
    };

    interface RequestOrganizationMember {
        organization_id: string;
    }
    // type RequestOrganizationRepository = RequestOrganizationMember & {
    //     organization_name: string,
    // }

    interface RequestOrganizationRepository {
        name: string;
    }

    interface RequestOrganizationCheckRepositoryName {
        repository_name: string;
        organization_id: string;
        creator_id?: string;
    }

    interface ResponseOrganizationCheckRepositoryName {
        nameExist: boolean;
    }

    interface RequestOrganizationMemberAdd {
        organization_id: string;
        user_id: string;
    }

    interface ResponseOrganizationMember {
        id: string;
        user_id: string;
        organization_id: string;
        role: string;
        name: string;
        user_name: string;
    }

    interface ResponseFriend {
        id: string;
    }

    interface RequestNameById {
        id: string;
    }

    interface RequestOnlyOwnerID {
        owner: string;
    }

    interface ResponseOptionsAuthor {
        id: string;
        name: string;
        // type: string,
        type: AuthorType;
    }

    // RequestRepositoryNew 创建新创库的请求参数
    interface RequestRepositoryNew {
        // owner: string,
        author_id: string;
        name: string;
        description: string;
        is_private: RepositoryType;
        author_type: AuthorType; // author_type repo author 类型, 可以是user或者org
        author_name: string;
    }

    interface ResponseRepository {
        owner: string;
        id: string;
        name: string;
        description: string;
        is_private: RepositoryType;
        init: number;
        binary_id: string;
        fork_from: string;
        author_type: string;
        author_name: string;
        releaseCount?: number;
        fork_repository?: ResponseRepository;
        date?: number;
    }

    interface ResponseRepositoryCommit {
        author: string;
        message: string;
        commit: string;
        date: number;
    }

    // commit detail and diff
    interface ResponseRepositoryCommitDetail {
        header_info: {
            id: string;
            oid: string;
            object_id: string;
            parent: string;
            tree: string;
            payload: string;
            message: string;
            author: string;
            committer: string;
        },
        diff: DiffResult[],
    }

    interface ServiceResponseUserData {
        userId: string;
        name: string;
        email: string;
        date: number;
        owner?: string;
    }

    interface ResponseUserGetIdByName {
        ownerId: string;
        id: string;
        type: string;
        message: string;
    }

    type RequestRepositoryMemberGet = RequestRepositoryMemberAdd;

    interface RequestRepositoryMemberAdd {
        repository_id: string;
        user_id: string;
        // user_name: string,
    }

    interface RequestRepositoryMemberList {
        repository_id: string;
    }

    type RequestRepositoryMemberDelete = RequestRepositoryMemberAdd & {
        id: string;
    };
    type RequestRepositoryMemberChangeRole = RequestRepositoryMemberAdd & {
        id: string;
        role: string;
    };

    interface RequestRepositoryVerify {
        id: string;
        user_id: string;
    }

    interface ResponseRepositoryMember {
        id: string;
        repository_id: string;
        user_id: string;
        role: string;
        user_name: string;
    }

    interface ResponseRepositoryPushHead {
        refs: IRef[];
    }

    type ResponseRepositoryFetchHead = ResponseRepositoryPushHead;

    // type RequestRepositoryFetchHead = RequestRepositoryPushHead
    interface RequestRepositoryFetchHead {
        id: string;
        user_id: string;
        branch?: string;
    }

    interface ResponseRepositoryPush {
        msg: string;
        file_id: string;
        device_id: string;
    }

    interface RequestTargetCommonResponse<S> {
        err: boolean;
        status: number;
        data?: S;
        msg: string;
    }

    interface ResponseCommitInfo {
        commit: string;
        author: string;
        date: number;
        message: string;
    }

    interface ResponseRepositoryHome {
        repository: ResponseRepository;
        commit_count: number;
        branches: string[];
        last_commit?: ResponseCommitInfo;
        releaseCount?: number;
        is_setting: boolean;
        star_count: number;
    }

    interface RequestRepositoryDelivery {
        owner_id: string;
        // repository_id: string,
        organization_id: string;
        new_member_id: string;
        new_member_name: string;
    }

    interface RequestRepositoryRemoteDeleteByOwner {
        repository_id: string;
    }

    interface RequestRepositoryStateSwitch {
        owner: string;
        id: string;
        action: SetStateType;
        // action: string
    }

    interface RepositoryFileMessage {
        file_data: FileData[];
        readme?: RepositoryFileReadme;
    }
    interface RepositoryFileReadme {
        content: string;
        type: string;
    }

    type FileData = ResponseFileCommit & {
        fileType: string;
        file: string;
    };

    interface ResponseRepositoryAnalysisItem {
        code: number;
        type: string;
    }
    interface ResponseRepositoryAnalysis {
        data: ResponseRepositoryAnalysisItem[];
    }

    interface RequestRepositoryReleaseAdd {
        id: string;
        user_id: string;
        tag_name: string;
        tag_target: string;
        title: string;
        content: string;
        owner: string;
    }

    interface RequestRepositoryReleaseDelete {
        release_id: string;
    }

    interface RequestRepositoryReleaseEdit {
        tag_name: string;
        title: string;
        content: string;
        id: string;
    }

    interface RequestRepositoryReleaseDetail {
        id: string;
        tag_name: string;
    }

    interface RequestRepositoryReleaseDownload {
        file_id: string;
        repo_owner: string;
        repo_id: string;
        repo_name: string;
        tag_name: string;
    }

    interface ResponseRepositoryRelease {
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

    interface ResponseRepoWikiHome {
        data: ResponseRepoWikiPage[];
        page: ResponseRepoWikiPage;
    }

    interface ResponseRepoWikiPage {
        content: string;
        date: number;
        id: string;
        publisher_id: string;
        title: string;
    }

    interface RequestWikiPageHome {
        id: string;
        title?: string;
    }

    interface RequestRepositoryWikiPageList {
        id: string;
    }

    interface RequestRepositoryWikiPageNew {
        user_id: string;
        title: string;
        content: string;
        id: string;
    }

    interface RequestRepositoryWikiDetail {
        id: string;
        title: string;
    }

    interface RequestRepositoryWikiEdit {
        title: string;
        content: string;
        id: string;
        wiki_id: string;
    }

    interface RequestRepositoryWikiDelete {
        wiki_id: string;
    }

    interface RequestRepositoryHome {
        owner?: string;
        id: string;
        branch: string;
    }

    interface RequestRepositoryFork {
        owner?: string;
        id: string;
        user_id: string;
        ood: string;
    }

    interface ResponseRepositoryStar {
        number: number;
        forkNumber: number;
        stared: boolean;
    }

    interface ResponseFile {
        fileType: string;
        fileData?: {
            content: ResponseFileLineContent[];
            bigFile: boolean;
            notSupport: boolean;
            info: ResponseFileCommit & {
                fileSize: number;
            };
        };
        dirData?: {
            data: ResponseRepoFile[];
            readme?: RepositoryFileReadme;
        };
    }

    interface ResponseCompareFile {
        notSupport: boolean;
        content: DiffLineData[];
    }

    interface CompareCommit {
        commit: string;
        message: string;
    }

    interface ResponseMergeCompare {
        commits: CompareCommit[];
        diff: DiffResult[];
    }

    interface RequestRepositoryList {
        repo_name?: string;
        page_index?: number;
        page_size?: number;
    }

    interface RequestRepositoryFind {
        repo_name: string;
        author_name: string;
    }

    interface RequestRepositoryBranches {
        owner: string;
        id: string;
    }

    interface RequestRepositoryDelete {
        owner: string;
        id: string;
    }

    interface RequestRepositoryDrop {
        owner?: string;
        id: string;
        user_id: string;
    }

    interface RequestRepositoryMerges {
        owner: string;
        id: string;
    }

    interface RequestRepositoryReleaseList {
        id: string;
    }

    interface RequestRepositoryStarStatus {
        owner?: string;
        id: string;
        user_id: string;
    }

    interface RequestRepositoryStarAdd {
        owner?: string;
        id: string;
        user_id: string;
    }

    interface RequestRepositoryStarDelete {
        owner?: string;
        id: string;
        user_id: string;
    }

    interface RequestOrganizationByName {
        organization_name: string;
    }

    interface RequestOrganizationList {
        organization_name?: string;
        page_index?: number;
        page_size?: number;
    }

    interface RequestSerivceRepositoryList {
        repo_name?: string;
        page_index?: number;
        page_size?: number;
        user_id?: string;
    }

    interface RequestUserList {
        user_name?: string;
        page_index?: number;
        page_size?: number;
    }

    interface RequestGetIdByName {
        user_name: string;
    }

    interface RepositoryFindRequest {
        owner: string;
        name: string;
    }

    interface RequestIssueCreate {
        owner: string;
        id: string;
        title: string;
        content: string;
        user_id: string;
        author_name: string;
        name: string;
    }


    interface RequestRepoIssueComment {
        issue_id: string;
        id?: string;
        author_name: string;
        name: string;
        content: string;
        user_id: string;
    }

    interface RequestRepoIssueClose {
        issue_id: string;
        author_name: string;
        name: string;
    }
}

export interface requestFileData {
    name: string;
    author_name: string;
    hash: string;
    owner?: string;
    file_name: string;
    path?: string;
    // id: string,
    branch: string;
}

export interface ResponseCommit {
    owner: string;
    id: string;
    oid: string;
    message: string;
    parent: string;
    // author: Object;
    author: string,
    date: number;
    // committer: Object;
    committer: string;
}

export interface RequestCommits {
    name: string;
    author_name: string;
    owner: string;
    id: string;
    branch: string;
}

export interface responseUser {
    id: string;
    name: string;
    email: string;
    date: number;
    owner_id: string;
}

export interface Issue {
    status: string;
}

export interface ResponseIssue {
    topic: ResponseIssueItem;
    issues?: ResponseIssueItem[];
}

export interface ResponseIssueItem {
    id: string;
    object_id: string;
    author_name: string;
    name: string;
    user_name: string;
    user_id: string;
    repo_id?: string;
    title: string;
    content: string;
    status: string;
    date: number;

    topic_comment_length?: number;
}

export interface ResponseIssueList {
    list: ResponseIssueItem[];
    issues?: ResponseIssueItem[];
    open: number;
    close: number;
    mine: number;
    other: number;
}

export interface ResponseRepositoryIssues {
    issues: ResponseIssueItem[];
    close_count: number;
    open_count: number;
}

export interface ResponseMerge {
    author_name: string;
    name: string;
    user_name: string;
    id: string;
    title: string;
    user_id: string;
    repo_id: string;
    repository_owner_id: string;
    origin_branch: string;
    target_branch: string;
    merge_type: string;
    status: string;
    date: number;
}

export interface ResponseMergeList {
    merge_list: ResponseMerge[];
    open: number;
    close: number;
    mine: number;
    other: number;
}

export interface RequestRepositoryMergeCompare {
    owner?: string;
    id: string;
    target: string;
    origin: string;
}

export interface RequestRepositoryMergeCreate {
    repository_owner_id: string;
    id: string;
    target: string;
    origin: string;
    title: string;
    mergeType: string;
}

export type RequestRepositoryMergeCompareFile = RequestRepositoryMergeCompare & {
    fileName: string;
};

export interface RequestRepositoryMergeDetail {
    author_name: string;
    name: string;
    merge_id: string;
}

export interface RequestRepositoryCommit {
    owner?: string;
    id: string;
    commitId: string;
}



export interface RequestIssueDetail {
    owner?: string;
    id?: string;
    issue_id: string;
    author_name: string;
    name: string;
}

export interface RequestRepoIssue {
    owner?: string;
    id: string;
    author_name: string;
    name: string;
}

export interface RequestUserInit {
    owner: string;
    name: string;
    email: string;
}

export interface RequestOrganizationCreate {
    name: string;
    email: string;
}

export interface RequestRepositoryPush {
    id: string; // 仓库object id
    binary: string;
    runtimeDeviceId: string;
    user_id: string; // 上传的用户
}

export interface RequestRepositoryPushV2 {
    id: string; // 仓库object id
    packFileId: string;
    refs: string;
    runtimeDeviceId: string;
    user_id: string; // 上传的用户
}

export interface RequestRepositoryPushHead {
    id: string; // 仓库object id
    branch?: string;
    user_id: string; // 上传的用户
}

export interface RequestRepositoryFetch {
    id: string; // 仓库object id
    hash: string;
    localHash: string;
    ref: string;
    user_id: string; // 上传的用户
}

export interface RequestRepositoryTrans {
    id: string; // 仓库object id
    binary: string;
    runtimeDeviceId: string;
    target: string;
}

export interface RequestUserSetting {
    userId: string;
    name: string;
    owner?: string;
    email: string;
}

export interface ResponseFileCommit {
    commit: string;
    author: string;
    date: number | string;
    message: string;
}
export interface ResponseFileLineContent {
    line: number;
    content: string;
}

export interface ResponseCommitShowDiff {
    data: DiffLineData[];
    commitId: string;
    headerInfo: ResponseFileCommit;
}

export interface ResponseFineRepository {
    repo: string;
    device: string;
}

export interface ResponseAddFile {
    err: boolean;
    msg: string;
    file_id?: string;
}

export interface ResponseCheckUser {
    userInit: boolean;
    user?: ServiceResponseUserData;
}

export interface GetObjectResponse<S> {
    err: boolean;
    object?: S;
    message: string;
}



export interface DiffResult {
    fileName: string;
    diffType?: string;
    count: string;
}

export interface DiffLineData {
    pathName: string;
    title: string;
    data: Array<string>;
}

export interface DiffParseResponse {
    data: DiffLineData[];
}
