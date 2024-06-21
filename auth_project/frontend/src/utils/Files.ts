import fs from 'fs';

export class File {
    path: string;

    constructor(path: string) {
        this.path = path;
    }

    openFile(): string | null {
        try {
            const data: string = fs.readFileSync(this.path, 'utf8');
            return data;
        } catch (err) {
            console.error(`Error occurred while opening the file: \n${err}`);
            return null;
        }
    }

    writeFile(data: string): void {
        fs.writeFile(this.path, data, (err) => {
            if (err) {
                console.error(err);
            }
        });
    }

    appendToFile(data: string): void {
        fs.appendFile(this.path, data, (err) => {
            if (err) {
                console.error(`Error occurred while appending the data to the file: \n${err}`);
            }
        });
    }

    deleteFile(): void {
        fs.unlink(this.path, (err) => {
            if (err) {
                console.error(`Error occurred while deleting the file: \n${err}`);
            }
        });
    }
}