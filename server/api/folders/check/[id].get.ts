import { getServerSession, getToken } from '#auth';
import prisma from './../../../db/prisma';

export default defineEventHandler(async (event) => {
    const session = await getServerSession(event)
    if (!session) {
        setResponseStatus(event, 404)
        return event
    }

    const token = await getToken({ event })

    const queryParams = getRouterParams(event)
    const folderId = queryParams.id

    if (token?.sub) {
        try {
            const folder = await prisma.folder.findFirst({
                where: {
                    id: +folderId
                }
            });

            if (folder?.userId === +token.sub) {
                setResponseStatus(event, 200)
                return event
            }
            setResponseStatus(event, 401)
            return event
        } catch (error) {
            setResponseStatus(event, 404)
            return event
        }
    } else {
        setResponseStatus(event, 404)
        return event
    }
})