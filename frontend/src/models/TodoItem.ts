export class TodoItem {
    public id: number;
    public title: string;
    public status: boolean;

    constructor(title: string, id?: number, status?: boolean) {
        this.title = title;
        
        if(status == undefined)
            this.status = false;
        else 
            this.status = status;

        if(id == undefined)
            this.id = 0;
        else 
            this.id = id;
    }
}