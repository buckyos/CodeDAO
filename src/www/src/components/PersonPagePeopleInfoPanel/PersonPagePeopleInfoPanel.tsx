import React, { useEffect, useMemo, useState } from 'react';
import styles from './PersonPagePeopleInfoPanel.module.less';
import { Button } from 'antd';
import UserCommonInfo from '@src/components/UserCommonInfo/UserCommonInfo';
import Suber from '@src/assets/images/personal_page/subscribers.png';
import Subing from '@src/assets/images/personal_page/subscribings.png';
import IntroIcon from '@src/assets/images/personal_page/intro.png';
import { PlusSquareOutlined, MailOutlined } from '@ant-design/icons';
import { getPubSubPeopleList, subscribePublisher } from '@src/apis/user';
import { useTranslation } from 'react-i18next';
import { message } from 'antd';

interface Props {
	peopleId: string;
	isSelf: boolean;
}

export default function PersonPagePeopleInfoPanel({ peopleId, isSelf }: Props) {
    const { t } = useTranslation();
    const [subLoadings, setSubLoadings] = useState(false);
    const [subscribers, setSubscribers] = useState(0);
    const [subscribings, setSubscribings] = useState(0);
    const [subDisable, setSubDisable] = useState(false);

    // 查询people列表
    const queryPeopleList = async () => {
        const ret = await getPubSubPeopleList(peopleId);
        if (ret) {
            setSubscribers(ret.publishers.length);
            setSubscribings(ret.subscribers.length);
            if (isSelf) {
                localStorage.setItem('subscribings', JSON.stringify(ret.subscribers));
            } else {
                const getValue = localStorage.getItem('subscribings');
                if (getValue) {
                    // 存储过自己订阅的列表
                    const subscribings = JSON.parse(getValue) as string[];
                    if (subscribings.includes(peopleId)) setSubDisable(true);
                }
            }
        }
    };
    // 订阅新人
    const addPublisher = async () => {
        setSubLoadings(true);
        const ret = await subscribePublisher(peopleId);
        if (ret.err) {
            // 订阅失败
            message.error(`${t('error.subscribe.fail')}`);
        } else {
            message.success(`${t('success.subscribe')}`);
            setSubDisable(true);
        }

        console.log('result:', ret);
        setSubLoadings(false);
    };

    useEffect(() => {
        queryPeopleList();
        const timer = setInterval(() => {
            queryPeopleList();
        }, 10000);
        return () => clearInterval(timer);
    }, [peopleId]);
    return (
        <div className={styles.box}>
            <div className={styles.userBox}>
                <UserCommonInfo
                    peopleId={peopleId}
                    introStr={peopleId}
                    iconHeight="88px"
                    iconWidth="88px"
                />
            </div>
            <div className={styles.pubsubBox}>
                <div className={styles.itemBox}>
                    <img className={styles.icon} src={Suber} />
                    <div className={styles.itemTxt}> {subscribers} Subscribers </div>
                </div>
                <div className={styles.itemBox}>
                    <img className={styles.icon} src={Subing} />
                    <div className={styles.itemTxt}> {subscribings} Subscribings </div>
                </div>
            </div>

            <div className={styles.itemBox}>
                <img className={styles.icon} src={IntroIcon} />
                <div className={styles.itemTxt}> a breif intro </div>
            </div>
            {!isSelf && (
                <div className={styles.btns}>
                    <Button
                        icon={<PlusSquareOutlined />}
                        size="middle"
                        onClick={addPublisher}
                        loading={subLoadings}
                        disabled={subDisable}
                    >
						Subscribe
                    </Button>
                    <Button
                        icon={<MailOutlined />}
                        size="middle"
                        style={{ marginLeft: '10px' }}
                        disabled
                    >
						Message
                    </Button>
                </div>
            )}
        </div>
    );
}
