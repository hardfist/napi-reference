import { deadlock} from '../index.js'
const locker = new deadlock.DeadLocker();

locker.bip();
locker.bip();