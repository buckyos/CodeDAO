import React, { useEffect } from 'react';

import { useParams } from 'react-router-dom';
import { Spin } from 'antd';
import {
    repositoryChildPath,
    useCleanSwitchPath
} from '../../../stores/repository';
import RepoFiles from '../../RepoFiles/RepoFiles';
import { useRepoFile } from '../../RepoFile/RepoFile';
import { useSetRecoilState } from 'recoil';
import { getBranch } from '../../../utils';

// RepoTree 仓库下的文件树
const RepoTree: React.FC = () => {
    const { owner, object_id } = useParams<RepoUrlParams>();
    const setRepositoryChildPath = useSetRecoilState(repositoryChildPath);
    const cleanRepositoryPathData = useCleanSwitchPath();
    // const [branch] = useRecoilState(repositoryCurrentBranchAtom)
    const branch = getBranch();

    const { data, loading, fileFullPath } = useRepoFile({
        name: object_id,
        owner: owner,
        author_name: owner,
        file_name: '',
        path: '',
        hash: '',
        branch: branch
    }, 'tree', branch);

    useEffect(() => {
        let isSubscribed = true;
        if (isSubscribed) {
            setRepositoryChildPath(fileFullPath);
            cleanRepositoryPathData();
        }
        return () => {isSubscribed = false;};
    }, [fileFullPath]);

    if (loading || data === undefined ) {
        return <Spin />;
    }

    const fileData:RepoFileList = {
        files: [],
        readme: {
            content: '',
            type: ''
        }
    };
    if (data.fileType === 'dir' && data.dirData) {
        fileData.files = data.dirData.data;
        if (data.dirData.readme) {
            fileData.readme = data.dirData.readme;
        }
    }

    return <RepoFiles data={ fileData }/>;
};

export default RepoTree;
