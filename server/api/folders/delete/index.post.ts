import { getServerSession, getToken } from '#auth';
import prisma from './../../../db/prisma';

export default defineEventHandler(async (event) => {
    const session = await getServerSession(event)
    if (!session) {
        setResponseStatus(event, 404)
        return event
    }

    const token = await getToken({ event })

    const data = await readBody(event)

    if (token?.sub) {
        try {
            const folder = await prisma.folder.findUnique({
                where: {
                    id: +data.data.id,
                },
                select: {
                    userId: true,
                },
            });

            if (folder && folder.userId === +token.sub) {
                await prisma.folder.delete({
                    where: {
                        id: +data.data.id,
                    }
                });
                setResponseStatus(event, 200)
                return event
            } else {
                setResponseStatus(event, 401)
                return event
            }
        } catch (error) {
            setResponseStatus(event, 500)
            return event
        }
    } else {
        setResponseStatus(event, 404)
        return event
    }
})