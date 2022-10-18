import * as cyfs from 'cyfs-sdk';

export const DEC_ID_BASE_58 = '9tGpLNnS1JhPqjnorwHMc4veZdUMe5qfe67a7hfKVu7r';

export const DEC_ID = cyfs.ObjectId.from_base_58(DEC_ID_BASE_58).unwrap();

export const APP_NAME = 'cyfs-git';

export const APP_OPEN_URL =
	'cyfs://o/95RvaS5WnZknp7FR3GwihY3wyy32umXf417Zugk1exGN/index.html';

export const PUBSUB_CENTER_DEC_ID_STR =
	'9tGpLNnKReYwVv6HMQxtkAxA9N627tLJ4s2d8qa5AyW9';

export const PUBSUB_CENTER_DEC_ID = cyfs.ObjectId.from_base_58(
    PUBSUB_CENTER_DEC_ID_STR
).unwrap();
