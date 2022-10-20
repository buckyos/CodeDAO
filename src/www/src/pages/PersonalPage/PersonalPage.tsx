import React, { useEffect, useMemo, useState, useRef } from 'react';
import styles from './PersonalPage.module.less';
import PersonPagePeopleInfoPanel from '@src/components/PersonPagePeopleInfoPanel/PersonPagePeopleInfoPanel';
import PersonPageCollectibles from '@src/components/PersonPageCollectibles/PersonPageCollectibles';
import PersonPageDecApps from '@src/components/PersonPageDecApps/PersonPageDecApps';
import PersonPageActivities from '@src/components/PersonPageActivities/PersonPageActivities';
import { checkStack } from '@src/utils/cyfs_helper/stack_wraper';
import { useParams, useHistory } from 'react-router-dom';
import { PubSubObject } from '@src/types';
import { queryPubSubRecords } from '@src/apis/user';
import * as cyfs from 'cyfs-sdk';

export default function PersonalPage() {
    const [isSelf, setIsSelf] = useState(true);
    const [listData, setListData] = useState<PubSubObject[]>([]);
    const [startIndex, setStartIndex] = useState(0);
    const PAGE_SIGE = 5;
    const urlParam = useParams<{ people_id: string }>();
    const timer = useRef<NodeJS.Timer>();
    const currentPeopleId = useMemo(() => {
        setStartIndex(0);
        setListData([]);
        const isSelf = urlParam.people_id === 'self';
        console.log('isSelf>>>>>', isSelf);
        setIsSelf(isSelf);
        return isSelf ? checkStack().checkOwner().to_base_58() : urlParam.people_id;
    }, [urlParam.people_id]);

    // 滚动到底部翻页
    const handleScroll = (e: React.UIEvent<HTMLElement>) => {
        clearTimeout(timer.current!);
        timer.current = setTimeout(() => {
            const clientHeight = e.currentTarget.clientHeight;
            if (e.currentTarget.scrollHeight - e.currentTarget.scrollTop === clientHeight) {
                // 到达底部
                setStartIndex(startIndex + 1);
            }
        }, 500);
    };

    // 查询列表记录
    const queryRecordsList = async (startIndex: number, pageSize: number) => {
        const targetId = cyfs.PeopleId.from_base_58(currentPeopleId).unwrap().object_id;
        const queryArr = await queryPubSubRecords(targetId, startIndex, pageSize);
        if (queryArr) {
            const list = [...listData, ...queryArr];
            list.sort((a, b) => (a.createdTimestamp as number) - (b.createdTimestamp as number));
            setListData(list);
        }
    };

    useEffect(() => {
        console.log(`query records startIndex = ${startIndex}`);
        queryRecordsList(startIndex, PAGE_SIGE);
    }, [startIndex, urlParam.people_id]);
    return (
        <div className={styles.box}>
            {/* <h1>Personal Page</h1> */}
            <div className={styles.content}>
                <div className={styles.leftBox}>
                    <div className={styles.title}>Activities</div>
                    <div className={styles.activityBox} onScroll={handleScroll}>
                        <PersonPageActivities listData={listData} />
                    </div>
                    {/* <Button onClick={notifyCreateAritcle}>notifyCreateAritcle</Button>
					<Button onClick={getPeopleList}>getPeopleList</Button>
					<Button onClick={() => addPublisher()}>addPublisher</Button>
					<Button onClick={queryPubSubRecords2}>queryPubSubRecords</Button> */}
                </div>
                <div className={styles.rightBox}>
                    <PersonPagePeopleInfoPanel peopleId={currentPeopleId} isSelf={isSelf} />
                    <div className={styles.opBox}>
                        <PersonPageCollectibles />
                    </div>
                    <div className={styles.opBox}>
                        <PersonPageDecApps />
                    </div>
                </div>
            </div>
        </div>
    );
}
