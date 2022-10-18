import React, { useCallback, useEffect, useRef } from 'react';
import { modalStore } from '../../stores/modal';
import QRCode from 'qrcode';
import { Observer } from 'mobx-react';
import { Modal } from 'antd';
import CodeIcon from '@src/assets/images/code.png';
import styles from './Nft.css';

// NftModal
// nft 二维码弹窗
export const NftModal: React.FC = () => {
    const ref = useRef(null);

    useEffect(() => {
        if (ref.current) {
            modalStore.ref = ref;

            // 二维码 init
            QRCode.toCanvas(ref.current, [], function (error) {
                if (error) console.error(error);
                // console.log('success!');
            });
        }
    }, [ref]);

    return (
        <Observer>
            {() => (
                <Modal
                    title={modalStore.title}
                    visible={modalStore.show}
                    closable={false}
                    forceRender={true}
                    footer={null}
                    maskClosable={true}
                    onCancel={() => modalStore.close()}
                    style={{ display: 'flex', justifyContent: 'center' }}
                >
                    <div style={{ display: 'flex', justifyContent: 'center' }}>
                        <canvas ref={ref} id="qrcode"></canvas>
                    </div>
                </Modal>
            )}
        </Observer>
    );
};

export const NftIcon: React.FC<{ id: string }> = ({ id }) => {
    const onClickOpen = useCallback(() => {
        QRCode.toCanvas(modalStore.ref?.current, id, function (error) {
            if (error) console.error(error);
            modalStore.show = true;
            modalStore.title = id;
            console.log('success! QRCode');
        });
    }, [id]);

    return (
        <div className={styles.commitItemCode} onClick={onClickOpen}>
            <img src={CodeIcon} />
        </div>
    );
};
