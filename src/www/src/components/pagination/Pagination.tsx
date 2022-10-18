import React, { useState } from 'react';
import { classSet, getPageIndex } from '../../utils';
import styles from './Pagination.css';
import { useTranslation } from 'react-i18next';

export const PaginationBottom: React.FC<PaginationParam> = ({ total, pageSize, onChange }) => {
    const [index, setIndex] = useState(getPageIndex());
    const totalPage = Math.max(Math.ceil(total / pageSize) - 1, 0);
    const { t } = useTranslation();

    const pageChange = (type: string) => {
        const count = type === 'prev' ? index - 1 : index + 1;
        setIndex(count);
        onChange(count);
    };

    return (
        <div className={styles.paginationBox}>
            <button
                className={classSet([styles.prevPage, styles.paginationCount])}
                disabled={index === 0}
                onClick={() => {
                    pageChange('prev');
                }}
            >
                {t('pagination.previous')}
            </button>
            <button
                className={classSet([styles.nextPage, styles.paginationCount])}
                disabled={index === totalPage}
                onClick={() => {
                    pageChange('next');
                }}
            >
                {t('pagination.next')}
            </button>
        </div>
    );
};
