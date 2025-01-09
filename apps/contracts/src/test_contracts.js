import { exec } from "child_process";
import { promisify } from "util";
const execPromise = promisify(exec);

const main = async () => {
    console.log('🟡0')
    try {
        exec('echo "✅Hello, World!"', (err, stdout, stderr) => {
            if (err) {
                console.error('🔴', err);
                process.exit(1);
            }
            console.log('🟢', stdout);
        });
        const { stdout: stdout1, stderr: stderr1 } = await execPromise('make build');
        if (stderr1) {
            console.error('🔴', stderr1);
            throw new Error(stderr2);
        }
        console.log('🟢', stdout1);

        console.log('🟡1')
        const { stdout: stdout2, stderr: stderr2 } = await execPromise('make test');
        if (stderr2) {
            console.error('🔵', stderr2);
            throw new Error(stderr2);
        }
        console.log('🟢', stdout2);
    } catch (err) {
        console.error(err);
        process.exit(1);
    }
    process.exit(0);
}

await main();