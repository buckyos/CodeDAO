import React, { useCallback, useState } from 'react';
import { inviteModalStoreAtom } from '@src/stores/modal';
import { Modal, Input, AutoComplete, Spin, message } from 'antd';
import { requestLocal, useRequestShadow } from '@src/utils';
import styles from './Invite.css';
import { useRecoilState } from 'recoil';

export const InviteModal: React.FC<InviteModalProps> = ({ onConfirm }) => {
    const [value, setValue] = useState('');
    const [inviteModalStore, setInviteModalStore] = useRecoilState(inviteModalStoreAtom);

    const { data, loading } = useRequestShadow<{ data: ResponseFriend[] }>(
        () => requestLocal('friends', {}),
        [],
        'get friends faild'
    );

    const onClick = useCallback(() => {
        if (value === '') {
            return message.warn('empty value');
        }
        onConfirm(value);
    }, [value]);

    return (
        <Modal
            title={inviteModalStore.title}
            visible={inviteModalStore.show}
            closable={false}
            forceRender={true}
            footer={null}
            maskClosable={true}
            onCancel={() => setInviteModalStore({ ...inviteModalStore, show: false })}
            style={{ display: 'flex', justifyContent: 'center' }}
        >
            <div style={{ display: 'flex', justifyContent: 'center' }}>
                {loading && <Spin></Spin>}
                {!loading && (
                    <AutoComplete
                        dropdownClassName="certain-category-search-dropdown"
                        dropdownMatchSelectWidth={500}
                        style={{ width: 320 }}
                        onSelect={setValue}
                    >
                        <Input
                            onChange={(e) => setValue(e.target.value)}
                            size="large"
                            placeholder="Input the taget member's DID"
                        />
                    </AutoComplete>
                )}
                <div onClick={onClick} className={styles.addButton}>
                    Add Member
                </div>
            </div>
        </Modal>
    );
};
